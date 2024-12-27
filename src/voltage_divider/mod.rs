use crate::types::{current::Current, power::Power, resistance::Resistance, voltage::Voltage};
use crate::types::{Measurement, ParserError};
use iced::widget::{Button, Column, Container, Row, Rule, Scrollable, Text, TextInput};
use iced::{Color, Element, Fill};

#[derive(Debug, Clone)]
pub struct VoltageDivider {
    legs: Vec<Leg>,
}

impl Default for VoltageDivider {
    fn default() -> Self {
        let legs = vec![Leg::default(), Leg::default()];

        Self { legs: legs }
    }
}

#[derive(Debug, Clone)]
struct Leg {
    resistance_raw: String,
    voltage_raw: String,
    voltage: Result<Voltage, ParserError>,
    current: Result<Current, ParserError>,
    resistance: Result<Resistance, ParserError>,
    power: Result<Power, ParserError>,
}

impl Default for Leg {
    fn default() -> Self {
        Self {
            resistance_raw: String::new(),
            voltage_raw: String::new(),
            voltage: Err(ParserError::EmptyInput),
            current: Err(ParserError::EmptyInput),
            resistance: Err(ParserError::EmptyInput),
            power: Err(ParserError::EmptyInput),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputVoltageChanged(usize, String),
    InputResistanceChanged(usize, String),
    LegAdd,
    LegDelete(usize),
}

impl VoltageDivider {
    pub fn title(&self) -> String {
        String::from("Voltage Divider")
    }

    pub fn view(&self) -> Element<Message> {
        Column::new()
            .push(self.view_form())
            .push(self.view_result())
            .into()
    }

    fn view_result(&self) -> Element<Message> {
        fn format_measurement<T: Measurement, E>(data: Result<T, E>) -> (String, String, String) {
            match data {
                Ok(measurement) => (
                    measurement.get_value_nom(),
                    measurement.get_value_min(),
                    measurement.get_value_max(),
                ),
                Err(_) => ("N/A".to_string(), "N/A".to_string(), "N/A".to_string()),
            }
        }
        fn format_tol<T: Measurement, E>(data: Result<T, E>) -> (String, String, String, String) {
            match data {
                Ok(measurement) => (
                    measurement.get_tol_value_plus(),
                    measurement.get_tol_value_minus(),
                    measurement.get_tol_percent_plus(),
                    measurement.get_tol_percent_minus(),
                ),
                Err(_) => (
                    "N/A".to_string(),
                    "N/A".to_string(),
                    "N/A".to_string(),
                    "N/A".to_string(),
                ),
            }
        }

        let mut data: Vec<(String, Vec<Vec<String>>)> = Vec::new();
        for (id, leg) in self.legs.iter().enumerate() {
            let (voltage_nom, voltage_min, voltage_max) = format_measurement(leg.voltage.clone());
            let (voltage_tol_plus, voltage_tol_minus, voltage_tol_plus_p, voltage_tol_minus_p) =
                format_tol(leg.voltage.clone());

            let (current_nom, current_min, current_max) = format_measurement(leg.current.clone());
            let (current_tol_plus, current_tol_minus, current_tol_plus_p, current_tol_minus_p) =
                format_tol(leg.current.clone());

            let (resistance_nom, resistance_min, resistance_max) =
                format_measurement(leg.resistance.clone());
            let (
                resistance_tol_plus,
                resistance_tol_minus,
                resistance_tol_plus_p,
                resistance_tol_minus_p,
            ) = format_tol(leg.resistance.clone());

            let (power_nom, power_min, power_max) = format_measurement(leg.power.clone());
            let (power_tol_plus, power_tol_minus, power_tol_plus_p, power_tol_minus_p) =
                format_tol(leg.power.clone());

            let iter_data: Vec<Vec<String>> = vec![
                vec![
                    "Value nom".to_string(),
                    voltage_nom,
                    current_nom,
                    resistance_nom,
                    power_nom,
                ],
                vec![
                    "Value max".to_string(),
                    voltage_max,
                    current_max,
                    resistance_max,
                    power_max,
                ],
                vec![
                    "Value min".to_string(),
                    voltage_min,
                    current_min,
                    resistance_min,
                    power_min,
                ],
                vec![
                    "Tol plus".to_string(),
                    voltage_tol_plus,
                    current_tol_plus,
                    resistance_tol_plus,
                    power_tol_plus,
                ],
                vec![
                    "Tol minus".to_string(),
                    voltage_tol_minus,
                    current_tol_minus,
                    resistance_tol_minus,
                    power_tol_minus,
                ],
                vec![
                    "Tol plus, %".to_string(),
                    voltage_tol_plus_p,
                    current_tol_plus_p,
                    resistance_tol_plus_p,
                    power_tol_plus_p,
                ],
                vec![
                    "Tol minus, %".to_string(),
                    voltage_tol_minus_p,
                    current_tol_minus_p,
                    resistance_tol_minus_p,
                    power_tol_minus_p,
                ],
            ];
            let collect = (format!("R{}", id + 1), iter_data);

            data.push(collect);
        }

        self.view_table(data).into()
    }

    fn view_table(&self, table_data: Vec<(String, Vec<Vec<String>>)>) -> Element<Message> {
        const BORDER_WIDTH: u16 = 0;
        const FIRST_COLUMN_WIDTH: u16 = 110;

        fn create_text_cell(content: String) -> Element<'static, Message> {
            let text = Text::new(content).width(Fill);

            Container::new(text).padding(5).into()
        }

        fn create_table_row(
            cell_1: String,
            cell_2: String,
            cell_3: String,
            cell_4: String,
            cell_5: String,
        ) -> Element<'static, Message> {
            Row::new()
                .push(Rule::vertical(BORDER_WIDTH))
                .push(Container::new(create_text_cell(cell_1)).width(FIRST_COLUMN_WIDTH))
                .push(Rule::vertical(BORDER_WIDTH))
                .push(Text::new("").width(1)) // Double border line
                .push(Rule::vertical(BORDER_WIDTH))
                .push(create_text_cell(cell_2))
                .push(Rule::vertical(BORDER_WIDTH))
                .push(create_text_cell(cell_3))
                .push(Rule::vertical(BORDER_WIDTH))
                .push(create_text_cell(cell_4))
                .push(Rule::vertical(BORDER_WIDTH))
                .push(create_text_cell(cell_5))
                .push(Rule::vertical(BORDER_WIDTH))
                .height(30)
                .width(Fill)
                .into()
        }

        let mut table_sections = Vec::new();
        // header
        let header = Row::new()
            .push(Rule::vertical(BORDER_WIDTH))
            .push(Container::new(create_text_cell("".to_string())).width(50 + FIRST_COLUMN_WIDTH))
            .push(Rule::vertical(BORDER_WIDTH))
            .push(Text::new("").width(1)) // Double border line
            .push(Rule::vertical(BORDER_WIDTH))
            .push(create_text_cell("Voltage".to_string()))
            .push(Rule::vertical(BORDER_WIDTH))
            .push(create_text_cell("Current".to_string()))
            .push(Rule::vertical(BORDER_WIDTH))
            .push(create_text_cell("Resistance".to_string()))
            .push(Rule::vertical(BORDER_WIDTH))
            .push(create_text_cell("Power".to_string()))
            .push(Rule::vertical(BORDER_WIDTH))
            .push(Text::new("").width(15)) // padding for Scrollable
            .height(30)
            .width(Fill)
            .into();
        table_sections.push(Rule::horizontal(BORDER_WIDTH).into());
        table_sections.push(header);

        // data
        for (section_label, rows) in table_data {
            let mut row_elements = Vec::new();

            for row_cells in rows {
                let row = create_table_row(
                    row_cells[0].clone(),
                    row_cells[1].clone(),
                    row_cells[2].clone(),
                    row_cells[3].clone(),
                    row_cells[4].clone(),
                );
                row_elements.push(Rule::horizontal(BORDER_WIDTH).into());
                row_elements.push(row);
            }
            row_elements.push(Rule::horizontal(BORDER_WIDTH).into());

            let section_content = Column::from_vec(row_elements).width(Fill);

            let section_label_column = Column::new()
                .push(Rule::horizontal(BORDER_WIDTH))
                .push(
                    Container::new(create_text_cell(section_label))
                        .height(Fill)
                        .align_y(iced::Alignment::Center),
                )
                .push(Rule::horizontal(BORDER_WIDTH))
                .width(50);

            let section_row = Row::new()
                .push(Rule::vertical(BORDER_WIDTH))
                .push(section_label_column)
                .push(Rule::vertical(BORDER_WIDTH))
                .push(section_content)
                .push(Text::new("").width(15)) // padding for Scrollable
                .height(210);

            table_sections.push(section_row.into());
        }

        let table_layout = Column::from_vec(table_sections).padding([5, 0]).width(Fill);

        Scrollable::new(table_layout).height(Fill).into()
    }

    fn view_form(&self) -> Element<Message> {
        let mut elements = Vec::new();
        for (id, leg) in self.legs.iter().enumerate() {
            let label1_text = format!("R{}", id + 1);
            let label2_text = format!("U{}", id + 1);
            let delete = if id <= 1 { false } else { true };
            let under_text = match (&self.legs[id].resistance, &self.legs[id].voltage) {
                // Некорректный ввод сопротивления и напряжения
                (Err(ParserError::IncorrectInput(e1)), Err(ParserError::IncorrectInput(e2))) => {
                    format!(
                        "Resistance field error: {}; Voltage field error: {}",
                        e1, e2
                    )
                }
                // Некорректный ввод сопротивления, напряжение корректно
                (Err(ParserError::IncorrectInput(e1)), Ok(_)) => {
                    format!("Resistance field error: {}", e1)
                }
                // Сопротивление корректно, некорректный ввод напряжения
                (Ok(_), Err(ParserError::IncorrectInput(e2))) => {
                    format!("Voltage field error: {}", e2)
                }
                // Пустой ввод сопротивления и напряжения
                (Err(ParserError::EmptyInput), Err(ParserError::EmptyInput)) => {
                    String::from("Both resistance and voltage fields are empty.")
                }
                // Пустой ввод сопротивления, напряжение корректно
                (Err(ParserError::EmptyInput), Ok(_)) => String::from("Resistance field is empty."),
                // Сопротивление корректно, пустой ввод напряжения
                (Ok(_), Err(ParserError::EmptyInput)) => String::from("Voltage field is empty."),
                // Все корректно
                (Ok(_), Ok(_)) => String::from("All fields are correct."),
                // Пример по умолчанию
                _ => String::from("Example: 1k 5%"),
            };

            let field = self.create_input_field(
                id,
                label1_text,
                &leg.resistance_raw,
                label2_text,
                &leg.voltage_raw,
                under_text,
                delete,
            );
            elements.push(field);
        }

        let label = Container::new(Text::new("Add leg")).center_x(Fill);
        let button = Button::new(label)
            .on_press(Message::LegAdd)
            .width(Fill)
            .into();
        elements.push(button);

        Column::from_vec(elements)
            .padding([5, 0])
            .width(Fill)
            .into()
    }

    fn create_input_field<'a>(
        &self,
        leg_id: usize,
        label1_text: String,
        input1_value: &'a str,
        label2_text: String,
        input2_value: &'a str,
        under_text: String,
        delete_button_view: bool,
    ) -> Element<'a, Message> {
        let label1 = Text::new(label1_text)
            .height(30)
            .width(30)
            .align_y(iced::Alignment::Center);
        let input1 = TextInput::new("", input1_value)
            .on_input(move |s| Message::InputResistanceChanged(leg_id, s));
        let label2 = Text::new(label2_text)
            .height(30)
            .width(30)
            .align_y(iced::Alignment::Center);
        let input2 = TextInput::new("", input2_value)
            .on_input(move |s| Message::InputVoltageChanged(leg_id, s));
        let button1: Element<Message> = if delete_button_view == true {
            Button::new(Text::new("−").size(16))
                .on_press(Message::LegDelete(leg_id))
                .width(30)
                .height(30)
                .into()
        } else {
            Text::new("").width(30).into()
        };
        let button1 = Row::new().push(Text::new("").width(5)).push(button1);

        let row1 = Row::new()
            .push(label1)
            .push(input1)
            .push(Text::new("").width(50))
            .push(label2)
            .push(input2)
            .push(button1);

        let row2 = Row::new().push(Text::new("").width(30)).push(
            Text::new(under_text)
                .color(Color::from_rgb8(128, 128, 128))
                .size(12),
        );

        Column::new().push(row1).push(row2).into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputResistanceChanged(id, s) => {
                self.legs[id].resistance_raw = s;
                self.legs[id].resistance = self.legs[id].resistance_raw.parse::<Resistance>();
            }
            Message::InputVoltageChanged(id, s) => {
                self.legs[id].voltage_raw = s;
                self.legs[id].voltage = self.legs[id].voltage_raw.parse::<Voltage>();
            }
            Message::LegAdd => self.legs.push(Leg::default()),
            Message::LegDelete(id) => {
                let _leg = self.legs.remove(id);
            }
        }

        // кажется нужно очищать значения если нет пользовательского ввода
        for leg in &mut self.legs.iter_mut() {
            if leg.voltage_raw.is_empty() {
                leg.voltage = Err(ParserError::EmptyInput);
                leg.power = Err(ParserError::EmptyInput);
                leg.current = Err(ParserError::EmptyInput);
            }
            if leg.resistance_raw.is_empty() {
                leg.resistance = Err(ParserError::EmptyInput);
                leg.power = Err(ParserError::EmptyInput);
                leg.current = Err(ParserError::EmptyInput);
            }
        }

        let mut v1: Option<Voltage> = None;
        let mut v2: Option<Voltage> = None;
        let mut r_sum: Option<Resistance> = None;
        let mut empty_fields = false;

        for leg in self.legs.iter().rev() {
            match (leg.resistance.clone(), leg.voltage.clone()) {
                (Err(_), Err(_)) => {
                    v1 = None;
                    v2 = None;
                    r_sum = None;
                    empty_fields = true;
                }
                (Ok(r), Ok(v)) => {
                    v2 = Some(v);
                    r_sum = if let Some(rr) = r_sum {
                        Some(r + rr)
                    } else {
                        Some(r)
                    };
                }
                (Err(_), Ok(v)) => {
                    v1 = Some(v);
                }
                (Ok(r), Err(_)) => {
                    if v2.is_none() {
                        r_sum = if let Some(rr) = r_sum {
                            Some(r + rr)
                        } else {
                            Some(r)
                        };
                    }
                }
            }
        }

        // если второе напряжение не определено, то принимаем его за 0
        if v1.is_none() {
            v1 = Some(Voltage::default());
        }

        let current = if let (Some(v1), Some(v2), Some(r)) = (v1, v2, r_sum) {
            if empty_fields == true {
                None
            } else {
                Some((v2 - v1) / r)
            }
        } else {
            None
        };

        if current.is_some() {
            let mut pre_voltage = Voltage::default();

            for leg in &mut self.legs.iter_mut().rev() {
                match (&leg.voltage, current, &leg.resistance) {
                    (Ok(v), Some(c), Err(_)) => {
                        leg.resistance = Ok((*v - pre_voltage) / c);
                        leg.current = Ok(c);
                        pre_voltage = *v;
                    }
                    (Ok(v), Some(c), Ok(_)) => {
                        leg.current = Ok(c);
                        pre_voltage = *v;
                    }
                    (Err(_), Some(c), Ok(r)) => {
                        let v = (c * *r) + pre_voltage;
                        leg.voltage = Ok(v);
                        leg.current = Ok(c);
                        pre_voltage = v;
                    }
                    (_, None, _) => leg.current = Err(ParserError::EmptyInput),
                    _ => (),
                }
            }
        }
    }
}

pub fn help() -> (String, String) {
    let title = String::from("Voltage Divider");
    let text = String::from("
The program calculates parameters in a resistive voltage divider circuit. It allows you to define the characteristics of each leg of the divider and provides tools for customization.

#### Features and Interface
1. **Leg Configuration**:  
   - By default, the circuit starts with two legs.  
   - You can add additional legs using the **Add Leg** button.  
   - Each additional leg will have a `-` button on the right for easy deletion.

2. **Automatic Numbering**:  
   - Legs are numbered automatically, starting from 1, and renumbered dynamically after any additions or deletions.

3. **Input Fields for Each Leg**:  
   - For each leg, you can specify:  
      -- **Resistance**: The resistance of the leg (in ohms, Ω).  
      -- **Voltage**: The voltage at the leg relative to ground (not the voltage drop across the resistor).  

4. **Calculation Requirements**:  
   - All known fields must be filled in.  
   - At least one leg must be fully defined, meaning both **resistance** and **voltage** must be provided for that leg.

#### Data Input Format
##### Value Units
The input format supports values with units, similar to those used in Ohm's Law calculations. To specify a unit, append the unit prefix directly to the number:  
- Example: 12m represents 0.012R (milliohms).  

Supported unit prefixes:  
- **p** (pico, 10⁻¹²),  
- **n** (nano, 10⁻⁹),  
- **u** (micro, 10⁻⁶),  
- **m** (milli, 10⁻³),  
- **k** (kilo, 10³),  
- **M** (mega, 10⁶),  
- **G** (giga, 10⁹).

##### Uncertainty (Error Margins)
Input values can include error margins using the following formats:  
- Symmetrical error: 5% (±5% from the value),  
- Asymmetrical positive error: +5%,  
- Asymmetrical negative error: -5%,  
- Symmetrical error: +/-5%.

#### Results
Once all required parameters are defined, the results will be displayed in a table below the input fields. Calculations account for any defined error margins and unit conversions. The results include:  
- Voltage distribution across all legs,  
- Current through each resistor,  
- Power dissipated by each resistor.");

    (title, text)
}
