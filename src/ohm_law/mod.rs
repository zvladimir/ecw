use iced::widget::{Column, Container, Row, Rule, Text, TextInput};
use iced::{Alignment, Color, Element, Fill};

use crate::types::{current::Current, power::Power, resistance::Resistance, voltage::Voltage};
use crate::types::{Measurement, ParserError};

#[derive(Debug, Clone)]
pub struct OhmLaw {
    fields_enable: FieldsEnable,
    data_raw: OhmDataRaw,
    data: OhmData,
    calc_type: CalcType,
}

#[derive(Debug, Clone, Copy)]
enum CalcType {
    None, // None
    VCRP, // Input V, C; Calc R, P
    VRCP, // Input V, R; Calc C, P
    VPCR, // Input V, P; Calc C, R
    CRVP, // Input C, R; Calc V, P
    CPVR, // Input C, P; Calc V, R
    RPVC, // Input R, P; Calc V, C
}

impl Default for OhmLaw {
    fn default() -> Self {
        OhmLaw {
            fields_enable: FieldsEnable::default(),
            data_raw: OhmDataRaw::default(),
            data: OhmData::default(),
            calc_type: CalcType::None,
        }
    }
}

#[derive(Debug, Clone)]
struct FieldsEnable {
    voltage: bool,
    current: bool,
    resistance: bool,
    power: bool,
}

impl Default for FieldsEnable {
    fn default() -> Self {
        Self {
            voltage: true,
            current: true,
            resistance: true,
            power: true,
        }
    }
}

#[derive(Debug, Clone)]
struct OhmData {
    voltage: Result<Voltage, ParserError>,
    current: Result<Current, ParserError>,
    resistance: Result<Resistance, ParserError>,
    power: Result<Power, ParserError>,
}

impl Default for OhmData {
    fn default() -> Self {
        Self {
            voltage: Err(ParserError::EmptyInput),
            current: Err(ParserError::EmptyInput),
            resistance: Err(ParserError::EmptyInput),
            power: Err(ParserError::EmptyInput),
        }
    }
}

#[derive(Debug, Clone)]
struct OhmDataRaw {
    voltage: String,
    current: String,
    resistance: String,
    power: String,
}

impl Default for OhmDataRaw {
    fn default() -> Self {
        Self {
            voltage: String::new(),
            current: String::new(),
            resistance: String::new(),
            power: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputVoltageChanged(String),
    InputCurrentChanged(String),
    InputResistanceChanged(String),
    InputPowerChanged(String),
}

impl OhmLaw {
    pub fn title(&self) -> String {
        String::from("Ohm Law")
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputVoltageChanged(s) => {
                self.data_raw.voltage = s;
                self.data.voltage = self.data_raw.voltage.parse::<Voltage>();
            }
            Message::InputCurrentChanged(s) => {
                self.data_raw.current = s;
                self.data.current = self.data_raw.current.parse::<Current>();
            }
            Message::InputResistanceChanged(s) => {
                self.data_raw.resistance = s;
                self.data.resistance = self.data_raw.resistance.parse::<Resistance>();
            }
            Message::InputPowerChanged(s) => {
                self.data_raw.power = s;
                self.data.power = self.data_raw.power.parse::<Power>();
            }
        }

        self.determine_calctype();
        self.update_field_accessibility();
        self.calculating();
    }

    fn determine_calctype(&mut self) {
        let voltage_filled = !self.data_raw.voltage.trim().is_empty() && self.data.voltage.is_ok();
        let current_filled = !self.data_raw.current.trim().is_empty() && self.data.current.is_ok();
        let resistance_filled =
            !self.data_raw.resistance.trim().is_empty() && self.data.resistance.is_ok();
        let power_filled = !self.data_raw.power.trim().is_empty() && self.data.power.is_ok();

        match (
            voltage_filled,
            current_filled,
            resistance_filled,
            power_filled,
        ) {
            (true, true, _, _) => self.calc_type = CalcType::VCRP,
            (true, _, true, _) => self.calc_type = CalcType::VRCP,
            (true, _, _, true) => self.calc_type = CalcType::VPCR,
            (_, true, true, _) => self.calc_type = CalcType::CRVP,
            (_, true, _, true) => self.calc_type = CalcType::CPVR,
            (_, _, true, true) => self.calc_type = CalcType::RPVC,
            _ => self.calc_type = CalcType::None,
        }
    }

    fn update_field_accessibility(&mut self) {
        match self.calc_type {
            CalcType::VCRP => {
                self.fields_enable.resistance = false;
                self.fields_enable.power = false;

                self.data_raw.resistance.clear();
                self.data_raw.power.clear();
            }
            CalcType::VRCP => {
                self.fields_enable.current = false;
                self.fields_enable.power = false;

                self.data_raw.current.clear();
                self.data_raw.power.clear();
            }
            CalcType::VPCR => {
                self.fields_enable.current = false;
                self.fields_enable.resistance = false;

                self.data_raw.current.clear();
                self.data_raw.resistance.clear();
            }
            CalcType::CRVP => {
                self.fields_enable.voltage = false;
                self.fields_enable.power = false;

                self.data_raw.voltage.clear();
                self.data_raw.power.clear();
            }
            CalcType::CPVR => {
                self.fields_enable.voltage = false;
                self.fields_enable.resistance = false;

                self.data_raw.resistance.clear();
                self.data_raw.voltage.clear();
            }
            CalcType::RPVC => {
                self.fields_enable.voltage = false;
                self.fields_enable.current = false;

                self.data_raw.voltage.clear();
                self.data_raw.current.clear();
            }
            CalcType::None => self.fields_enable = FieldsEnable::default(),
        }
    }

    fn calculating(&mut self) {
        match self.calc_type {
            CalcType::VCRP => {
                if let (Ok(voltage), Ok(current)) =
                    (self.data.voltage.clone(), self.data.current.clone())
                {
                    self.data.resistance = Ok(voltage / current);
                    self.data.power = Ok(voltage * current);
                }
            }
            CalcType::VRCP => {
                if let (Ok(voltage), Ok(resistance)) =
                    (self.data.voltage.clone(), self.data.resistance.clone())
                {
                    let current = voltage / resistance;

                    self.data.current = Ok(current);
                    self.data.power = Ok(voltage * current);
                }
            }
            CalcType::VPCR => {
                if let (Ok(voltage), Ok(power)) =
                    (self.data.voltage.clone(), self.data.power.clone())
                {
                    let current = power / voltage;

                    self.data.current = Ok(current);
                    self.data.resistance = Ok(voltage / current);
                }
            }
            CalcType::CRVP => {
                if let (Ok(resistance), Ok(current)) =
                    (self.data.resistance.clone(), self.data.current.clone())
                {
                    let voltage = current * resistance;

                    self.data.voltage = Ok(voltage);
                    self.data.power = Ok(voltage * current);
                }
            }
            CalcType::CPVR => {
                if let (Ok(power), Ok(current)) =
                    (self.data.power.clone(), self.data.current.clone())
                {
                    let voltage = power * current;

                    self.data.voltage = Ok(voltage);
                    self.data.resistance = Ok(voltage / current);
                }
            }
            CalcType::RPVC => {
                if let (Ok(power), Ok(resistance)) =
                    (self.data.power.clone(), self.data.resistance.clone())
                {
                    let voltage = Voltage {
                        value: (power.value * resistance.value).sqrt(),
                        tolerance: None,
                    };
                    let current = Current {
                        value: (power.value / resistance.value).sqrt(),
                        tolerance: None,
                    };

                    self.data.voltage = Ok(voltage);
                    self.data.current = Ok(current);
                }
            }
            CalcType::None => (),
        }
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

        let (voltage_nom, voltage_min, voltage_max) = format_measurement(self.data.voltage.clone());
        let (voltage_tol_plus, voltage_tol_minus, voltage_tol_plus_p, voltage_tol_minus_p) =
            format_tol(self.data.voltage.clone());

        let (current_nom, current_min, current_max) = format_measurement(self.data.current.clone());
        let (current_tol_plus, current_tol_minus, current_tol_plus_p, current_tol_minus_p) =
            format_tol(self.data.current.clone());

        let (resistance_nom, resistance_min, resistance_max) =
            format_measurement(self.data.resistance.clone());
        let (
            resistance_tol_plus,
            resistance_tol_minus,
            resistance_tol_plus_p,
            resistance_tol_minus_p,
        ) = format_tol(self.data.resistance.clone());

        let (power_nom, power_min, power_max) = format_measurement(self.data.power.clone());
        let (power_tol_plus, power_tol_minus, power_tol_plus_p, power_tol_minus_p) =
            format_tol(self.data.power.clone());

        let data = vec![
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
        let result = self.view_table(data);

        Container::new(result).padding([1, 0]).into()
    }

    fn view_table(&self, data: Vec<Vec<String>>) -> Element<Message> {
        const RULE_WIDTH: u16 = 0;
        const COLUMN_FIRST_WIDTH: u16 = 110;

        fn text_output(s: String) -> Element<'static, Message> {
            let t = Text::new(s).width(Fill);

            Container::new(t).padding(5).into()
        }

        fn row_line(
            column1: String,
            column2: String,
            column3: String,
            column4: String,
            column5: String,
        ) -> Element<'static, Message> {
            Row::new()
                .push(Rule::vertical(RULE_WIDTH))
                .push(Container::new(text_output(column1)).width(COLUMN_FIRST_WIDTH))
                .push(Rule::vertical(RULE_WIDTH))
                .push(Text::new("").width(1)) // double rule line
                .push(Rule::vertical(RULE_WIDTH))
                .push(text_output(column2))
                .push(Rule::vertical(RULE_WIDTH))
                .push(text_output(column3))
                .push(Rule::vertical(RULE_WIDTH))
                .push(text_output(column4))
                .push(Rule::vertical(RULE_WIDTH))
                .push(text_output(column5))
                .push(Rule::vertical(RULE_WIDTH))
                .height(30)
                .width(Fill)
                .into()
        }

        let mut elements = Vec::new();
        // header
        let r = row_line(
            "".to_string(),
            "Voltage".to_string(),
            "Current".to_string(),
            "Resistance".to_string(),
            "Power".to_string(),
        );
        elements.push(Rule::horizontal(RULE_WIDTH).into());
        elements.push(r);
        elements.push(Rule::horizontal(RULE_WIDTH).into());
        elements.push(Text::new("").height(1).into());
        elements.push(Rule::horizontal(RULE_WIDTH).into());

        // data
        for d in data {
            let r = row_line(
                d[0].clone(),
                d[1].clone(),
                d[2].clone(),
                d[3].clone(),
                d[4].clone(),
            );
            elements.push(r);
            elements.push(Rule::horizontal(RULE_WIDTH).into());
        }

        Column::from_vec(elements)
            .padding([5, 0])
            .width(Fill)
            .into()
    }

    fn view_form(&self) -> Element<Message> {
        let under_text = match &self.data.voltage {
            Err(ParserError::IncorrectInput(e)) => e,
            _ => "Example: 10.5 +3% -7.6%",
        };
        let voltage_field = self.create_input_field(
            "Voltage",
            &self.data_raw.voltage,
            |s| Message::InputVoltageChanged(s),
            under_text,
            self.fields_enable.voltage,
        );
        let under_text = match &self.data.voltage {
            Err(ParserError::IncorrectInput(e)) => e,
            _ => "Example: 100m +1% -1%",
        };
        let current_field = self.create_input_field(
            "Current",
            &self.data_raw.current,
            |s| Message::InputCurrentChanged(s),
            under_text,
            self.fields_enable.current,
        );
        let under_text = match &self.data.resistance {
            Err(ParserError::IncorrectInput(e)) => e,
            _ => "Example: 10k 5%",
        };
        let resistance_field = self.create_input_field(
            "Resistance",
            &self.data_raw.resistance,
            |s| Message::InputResistanceChanged(s),
            under_text,
            self.fields_enable.resistance,
        );
        let under_text = match &self.data.power {
            Err(ParserError::IncorrectInput(e)) => e,
            _ => "Example: 1k 5%",
        };
        let power_field = self.create_input_field(
            "Power",
            &self.data_raw.power,
            |s| Message::InputPowerChanged(s),
            under_text,
            self.fields_enable.power,
        );

        Column::new()
            .push(voltage_field)
            .push(current_field)
            .push(resistance_field)
            .push(power_field)
            .into()
    }

    fn create_input_field<'a>(
        &self,
        label_text: &'a str,
        input_value: &'a str,
        on_input: impl Fn(String) -> Message + 'a,
        under_text: &'a str,
        enable: bool,
    ) -> Element<'a, Message> {
        // Константы для стилей
        const LABEL_WIDTH: u16 = 110;
        const FIELD_HEIGHT: u16 = 30;
        const LABEL_SIZE: u16 = 15;
        const INPUT_SIZE: u16 = 15;
        const UNDER_TEXT_SIZE: u16 = 12;
        const PADDING_ROW: [u16; 2] = [0, 0];
        const PADDING_COLUMN: [u16; 2] = [5, 0];
        const UNDER_TEXT_PADDING: [u16; 2] = [0, LABEL_WIDTH];

        // Метка
        let label = Text::new(label_text).size(LABEL_SIZE);
        let label = Container::new(label)
            .align_y(Alignment::Center)
            .width(LABEL_WIDTH)
            .height(FIELD_HEIGHT)
            .padding(PADDING_ROW);

        // Поле ввода
        let mut input = TextInput::new("", input_value).size(INPUT_SIZE);
        if enable == true {
            input = input.on_input(on_input);
        }
        let input = Container::new(input)
            .align_y(Alignment::Center)
            .width(Fill)
            .height(FIELD_HEIGHT);

        // Подсказка
        let under_text = Text::new(under_text)
            .size(UNDER_TEXT_SIZE)
            .color(Color::from_rgb8(128, 128, 128));
        let under_text = Container::new(under_text)
            .align_y(Alignment::Center)
            .padding(UNDER_TEXT_PADDING);

        // Компоновка
        Column::new()
            .push(Row::new().push(label).push(input))
            .push(under_text)
            .padding(PADDING_COLUMN)
            .into()
    }
}

pub fn help() -> (String, String) {
    let title = String::from("Ohm Law\n");
    let text = String::from("
The program performs calculations based on Ohm's Law: **U = I × R** and the power formula: **P = U × I**, where:  
- **U** — Voltage (volts, V),  
- **I** — Current (amperes, A),  
- **R** — Resistance (ohms, Ω),  
- **P** — Power (watts, W).

#### How to Use
1. Fill in any **two known fields** out of the four: voltage (**U**), current (**I**), resistance (**R**), or power (**P**).
2. After filling in two fields, the remaining fields will become read-only.
3. The results will be displayed in the table below.

If a parameter cannot be calculated, it will be marked as **N/A**.

#### Data Input Format
##### Value Units
Each input field supports values with units. To specify a unit, append the unit prefix directly to the number:  
- Example: 12m represents 0.012V (millivolts).  

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

#### Error Handling in Results
All input uncertainties are considered during calculations. The results will reflect the range of uncertainty based on the provided error margins.
");

    (title, text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculating_vcrp() {
        let mut ohm_law = OhmLaw::default();
        ohm_law.data.voltage = Ok(Voltage {
            value: 10.0,
            tolerance: None,
        });
        ohm_law.data.current = Ok(Current {
            value: 2.0,
            tolerance: None,
        });
        ohm_law.calc_type = CalcType::VCRP;

        ohm_law.calculating();

        assert_eq!(ohm_law.data.resistance.unwrap().get_nominal_value(), 5.0); // R = V / I
        assert_eq!(ohm_law.data.power.unwrap().get_nominal_value(), 20.0); // P = V * I
    }

    #[test]
    fn test_calculating_vrcp() {
        let mut ohm_law = OhmLaw::default();
        ohm_law.data.voltage = Ok(Voltage {
            value: 12.0,
            tolerance: None,
        });
        ohm_law.data.resistance = Ok(Resistance {
            value: 4.0,
            tolerance: None,
        });
        ohm_law.calc_type = CalcType::VRCP;

        ohm_law.calculating();

        assert_eq!(ohm_law.data.current.unwrap().get_nominal_value(), 3.0); // I = V / R
        assert_eq!(ohm_law.data.power.unwrap().get_nominal_value(), 36.0); // P = V * I
    }

    #[test]
    fn test_calculating_vpcr() {
        let mut ohm_law = OhmLaw::default();
        ohm_law.data.voltage = Ok(Voltage {
            value: 15.0,
            tolerance: None,
        });
        ohm_law.data.power = Ok(Power {
            value: 30.0,
            tolerance: None,
        });
        ohm_law.calc_type = CalcType::VPCR;

        ohm_law.calculating();

        assert_eq!(ohm_law.data.current.unwrap().get_nominal_value(), 2.0); // I = P / V
        assert_eq!(ohm_law.data.resistance.unwrap().get_nominal_value(), 7.5); // R = V / I
    }

    #[test]
    fn test_calculating_crvp() {
        let mut ohm_law = OhmLaw::default();
        ohm_law.data.current = Ok(Current {
            value: 2.0,
            tolerance: None,
        });
        ohm_law.data.resistance = Ok(Resistance {
            value: 5.0,
            tolerance: None,
        });
        ohm_law.calc_type = CalcType::CRVP;

        ohm_law.calculating();

        assert_eq!(ohm_law.data.voltage.unwrap().get_nominal_value(), 10.0); // V = I * R
        assert_eq!(ohm_law.data.power.unwrap().get_nominal_value(), 20.0); // P = V * I
    }

    #[test]
    fn test_calculating_cpvr() {
        let mut ohm_law = OhmLaw::default();
        ohm_law.data.current = Ok(Current {
            value: 3.0,
            tolerance: None,
        });
        ohm_law.data.power = Ok(Power {
            value: 27.0,
            tolerance: None,
        });
        ohm_law.calc_type = CalcType::CPVR;

        ohm_law.calculating();

        assert_eq!(ohm_law.data.voltage.unwrap().get_nominal_value(), 9.0); // V = P / I
        assert_eq!(ohm_law.data.resistance.unwrap().get_nominal_value(), 3.0); // R = V / I
    }

    #[test]
    fn test_calculating_rpvc() {
        let mut ohm_law = OhmLaw::default();
        ohm_law.data.resistance = Ok(Resistance {
            value: 4.0,
            tolerance: None,
        });
        ohm_law.data.power = Ok(Power {
            value: 64.0,
            tolerance: None,
        });
        ohm_law.calc_type = CalcType::RPVC;

        ohm_law.calculating();

        assert_eq!(ohm_law.data.voltage.unwrap().get_nominal_value(), 16.0); // V = sqrt(P * R)
        assert_eq!(ohm_law.data.current.unwrap().get_nominal_value(), 4.0); // I = sqrt(P / R)
    }

    #[test]
    fn test_calculating_none() {
        let mut ohm_law = OhmLaw::default();
        ohm_law.calc_type = CalcType::None;

        ohm_law.calculating();

        assert!(ohm_law.data.voltage.is_err());
        assert!(ohm_law.data.current.is_err());
        assert!(ohm_law.data.resistance.is_err());
        assert!(ohm_law.data.power.is_err());
    }
}
