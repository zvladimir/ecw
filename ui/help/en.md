# About ECW

ECW is a set of focused calculators for everyday electronics design. It grew out of the *TI Analog Engineer's Circuit Cookbook*: each page takes the few numbers you know, finds the ones you do not, and shows the result together with its **worst-case spread**.

Three ideas run through the whole program:

- **Fill in what you know.** You never choose a "mode" or press *Calculate*. Type values and the results appear at once. Fields that follow from the others fill themselves in and lock.
- **Tolerances are first-class.** A resistor is not 10 kΩ, it is 10 kΩ ±1 %. Wherever it makes sense you can add a tolerance to a value, and the answer comes back as a nominal value with a minimum and a maximum.
- **The help is a textbook.** Every calculator has a chapter that explains the electronics behind it: where the formulas come from, how to design with them step by step, what goes wrong in real circuits, and worked examples you can repeat in the program. The chapters point to the application notes and standards they rely on.

How this help is organised: *Quick start* explains the interface, *Foundations* recalls the laws used everywhere, then one chapter follows for every calculator (from the simple ones to the op-amp circuits), and the *Reference* at the end collects tables, formulas, standards and documents.

> **Tip.** The help opens in its own window (button **Help** or key **F1**), so you can keep it next to the calculator. Text in this window can be selected and copied, and underlined links open the cited documents in your browser.

# Quick start

## The window

The window has three areas:

- **Header.** The name **ECW** and four buttons: **Favorite** (adds the current calculator to the Favorites list), **Settings**, **Help** and **Donate**.
- **Left menu.** A search box and the list of calculators grouped by category (*Analog*, *Basic*). Click a category name to collapse it. Calculators marked as favorites are repeated in a **★ Favorites** block at the very top.
- **Calculator page.** Its title, a one-line description with the main formula, and then the page itself. The simple calculators are forms: **Design Inputs** (the fields you type into) and **Results**. The op-amp calculators are drawn as **schematics with the fields on them**, and have a second tab, **Plot**.

Everything you type is remembered between sessions, so the program opens exactly where you left it. The **⌫** button in the corner of a page clears its form.

## Entering numbers

Type plain numbers, optionally with an **SI prefix** as the last character. **Do not type units**: the field already knows that it expects volts, ohms and so on, and its label shows the unit in square brackets, for example *R1 [Ω]*.

| You type | Meaning                          |
|----------|----------------------------------|
| `4.7k`   | 4 700 (kilo, $10^3$)             |
| `2.2M`   | 2 200 000 (mega, $10^6$)         |
| `500m`   | 0.5 (milli, $10^{-3}$)           |
| `10u`    | 0.00001 (micro, $10^{-6}$)       |
| `100n`   | $10^{-7}$ (nano)                 |
| `1e-3`   | 0.001 (scientific notation)      |
| `-7`     | negative numbers are allowed     |

> **Careful with case.** `m` is *milli* and `M` is *mega*: `5m` is 0.005 but `5M` is five million. `G` is giga, `k` is kilo, `u` is micro, `n` is nano, `p` is pico. The prefixes are those of the SI (see *Reference*).

On form pages a grey line under each field shows how the program understood your input (for example *→ 4.7000 kΩ*) or, when the text is not valid, what is wrong with it. A red border marks a field with an error; on a schematic, hover over the field to read the message. The **×** button clears a field.

## Tolerances

Fields that describe components accept a tolerance after the number, separated by a space:

| You type        | Meaning                                              |
|-----------------|------------------------------------------------------|
| `10k 1%`        | 10 kΩ, ±1 %                                          |
| `10k +5% -1%`   | 10 kΩ, up to 5 % above and up to 1 % below           |
| `100 -10%`      | 100 Ω, only downward spread                          |

Which fields accept tolerances is stated in each chapter. In general resistors and reference voltages do, as do all quantities of the Ohm's Law and Voltage Divider pages. Signal ranges, gains, times and similar *exact* inputs do not: such a field turns red if you add a percentage.

When tolerances are present, the results show **Min** and **Max** values computed for the worst case. How this is done is explained in *Foundations*.

## Ranges

The op-amp calculators work with signal **ranges**: a sensor that gives −1 V to +1 V, an ADC that accepts 0 to 3.3 V. A range is typed into **one field** as two numbers separated by a semicolon, the lower one first:

| You type   | Meaning                                                    |
|------------|------------------------------------------------------------|
| `-1; 1`    | from −1 V to +1 V                                          |
| `0.05; 3.3`| from 50 mV to 3.3 V                                        |
| `5`        | Op-Amp (inverting, non-inverting): the single value 5 V    |
| `5`        | Difference amplifier, level shifters: the range 0…5 V      |

> **Note.** A single number means a single value on the two basic op-amp pages, and a range from 0 on the others. Write both ends when in doubt.

## Computed fields

On most pages you give *some* quantities and the program determines the rest. As soon as the input is sufficient, the fields you did not fill become **read-only**. On form pages they show the computed value in grey; on schematic pages they stay empty and dashed, and their values are in the results table. Clear one of your fields to make the others editable again.

If the numbers you entered contradict each other, or do not describe a physically possible circuit, the Results area shows a red message instead of a table, for example *Vout1 must be less than Vin*, together with what to change.

## Results and plots

Result tables have a **Copy table** button. It copies the table as tab-separated text, which pastes directly into a spreadsheet or a text document, with the number of decimals set in Settings.

The op-amp pages have a **Plot** tab: the output against the input. Hover over the plot to read values; the pointer sticks to the nearest line. Click a name in the legend to hide or show its line.

## Settings, favorites and search

- **Settings** (gear button) hold the **language** (English, Русский), the **theme** (Dark, Light or VS Code), the **zoom** (50–150 %) and the number of **decimal places** (0–9) in the results. Changes apply immediately.
- **Favorite** adds the open calculator to the ★ Favorites block of the left menu; click the star again to remove it.
- The **search box** filters the menu by title.

## Keyboard

| Key       | Action                                     |
|-----------|--------------------------------------------|
| `Tab`     | move to the next field                     |
| `Ctrl+C`  | copy the selected text of a field          |
| `F1`      | open this help                             |
| `Esc`     | close the Settings dialog or this window   |

# Foundations

This chapter recalls the handful of laws the calculators are built on. If you know them well, skip ahead to the chapter you need.

## Voltage, current, resistance

**Voltage** $U$ (volts, V) is the energy per unit charge needed to move charge between two points. **Current** $I$ (amperes, A) is the rate of flow of charge. **Resistance** $R$ (ohms, Ω) tells how much voltage is needed to push a given current through a component. **Ohm's law** states that for a resistor they are proportional:

$$U = I \cdot R$$

The reciprocal of resistance is **conductance** $G = 1/R$, measured in siemens (S). It is often more convenient for parallel circuits.

## Kirchhoff's laws

Two conservation laws let you analyse any circuit of resistors and sources:

- **Current law (KCL).** The currents flowing into a node add up to zero: charge does not pile up in a wire. For a node with currents $I_1 \dots I_n$ (into the node positive): $\sum I_k = 0$.
- **Voltage law (KVL).** The voltages around any closed loop add up to zero: the potential at a point does not depend on the path taken to reach it. $\sum U_k = 0$.

Every formula in the op-amp chapters is KCL written at the input node of the amplifier.

## Power

Electrical **power** (watts, W) is voltage times current, and with Ohm's law it takes three equivalent forms:

$$P = U \cdot I = I^2 \cdot R = \frac{U^2}{R}$$

In a resistor *all* of this power turns into heat. A resistor rated 0.25 W that dissipates 0.5 W will overheat, so power is checked as carefully as resistance. Power grows with the *square* of the current: doubling the current quadruples the heat. A common design rule is to keep the dissipation below half of the rating.

## Series and parallel connection

![Two resistors in series and in parallel](fig:series_parallel)

In a **series** connection the same current flows through both resistors and the voltages add up, so the resistances add:

$$R_{series} = R_1 + R_2$$

In a **parallel** connection both resistors have the same voltage and the currents add up, so the *conductances* add:

$$\frac{1}{R_{parallel}} = \frac{1}{R_1} + \frac{1}{R_2} \quad\Rightarrow\quad R_{parallel} = \frac{R_1 \cdot R_2}{R_1 + R_2}$$

A series combination is always *larger* than the largest resistor, a parallel combination is always *smaller* than the smallest one. If one resistor is much bigger than the other, the result is dominated by the smaller one in a parallel circuit and by the larger one in a series circuit.

## Superposition

In a circuit of resistors (any *linear* circuit) with several sources, the response is the **sum of the responses to each source alone**, with the other voltage sources replaced by short circuits. This is how the difference amplifier and the level shifters are analysed: the output is "the part due to the signal" plus "the part due to the reference".

## Tolerance and worst case

Real components are never exact. A "10 kΩ ±1 %" resistor may measure anywhere between 9.9 kΩ and 10.1 kΩ. A circuit built from such parts has an output that varies as well, and a good design must work for *every* combination of the allowed values.

The calculators find the **worst case**: the smallest and the largest value a result can take. Two methods are used.

- **Interval arithmetic.** Each value is carried as three numbers, *nominal*, *minimum* and *maximum*, and every operation is done on the bounds so that the result contains all possible outcomes. The sum of $a \in [a_{min}, a_{max}]$ and $b \in [b_{min}, b_{max}]$ lies in $[a_{min} + b_{min},\; a_{max} + b_{max}]$; a product or a quotient lies between the smallest and the largest of the four combinations of the bounds; division by an interval that contains zero is refused. The Ohm's Law page works this way.
- **Corner analysis.** Interval arithmetic is too pessimistic when the same component appears twice in a formula: in $V_{in} R_2 / (R_1 + R_2)$ a high $R_2$ raises the numerator and the denominator together, so the two effects partly cancel, which interval arithmetic cannot know. When a result changes monotonically with every input, its extremes lie at the **corners**, the combinations of the minimum and maximum of each input. For $n$ toleranced inputs the program evaluates the circuit at all $2^n$ corners and takes the lowest and the highest result. The Voltage Divider and all op-amp pages work this way, so their spread is exact, not just safe.

Worst case is the safe estimate, and a pessimistic one: real batches of components are statistically much better than the limits suggest, and it is unlikely that all parts sit at the unfavourable end at once. If you need a probabilistic estimate, use the worst case as an upper bound and refine it with a root-sum-square estimate or a Monte Carlo simulation.

In the tables the spread is shown as **Min**, **Max** and **Δ, %**, the deviation of the bounds from the nominal value (one number with ± when they are symmetric). The Ohm's Law and Voltage Divider pages show it in more detail:

| Row       | Meaning                                                     |
|-----------|-------------------------------------------------------------|
| Nominal   | the value for exactly nominal inputs                        |
| Min / Max | the smallest and the largest possible value                 |
| Δ + %     | how far above nominal the maximum lies, in percent          |
| Δ + val   | the same, as an absolute amount                             |
| Δ - %     | how far below nominal the minimum lies, in percent          |
| Δ - val   | the same, as an absolute amount                             |

## Preferred values: the E-series

Resistors are not manufactured in every value. Manufacturers offer values from the **preferred number series** of the standard **IEC 60063**, named **E6, E12, E24, E48, E96, E192** after the number of values per decade. The values are spaced geometrically:

$$R_k \approx 10^{k/N}, \qquad k = 0, 1, \dots, N-1$$

then rounded to two (E6–E24) or three (E48–E192) significant digits and repeated in every decade. For E12 the ratio between neighbours is $10^{1/12} \approx 1.21$, a step of about 21 %. The rounding is not always the obvious one: E24 contains 2.7, 3.0, 3.3 … 4.7, 8.2, which do not follow $10^{k/24}$ to two digits, and E192 lists 9.20 although $10^{185/192}$ rounds to 9.19. Use the standard table, not the formula, when the exact value matters.

![The E6, E12 and E24 series on a logarithmic scale](fig:eseries)

The spacing is chosen so that neighbouring values *just overlap* when the tolerance is taken into account: any resistance lies within the tolerance of some preferred value.

| Series | Values per decade | Typical tolerance |
|--------|-------------------|-------------------|
| E6     | 6                 | 20 %              |
| E12    | 12                | 10 %              |
| E24    | 24                | 5 % (and 1 %)     |
| E48    | 48                | 2 %               |
| E96    | 96                | 1 %               |
| E192   | 192               | 0.5 % and better  |

When a calculation gives an "ugly" value such as 26.36 kΩ, pick the nearest preferred value, or combine two resistors to get closer. The **Resistor Solver** does exactly that. On the op-amp pages you can type a standard value into a computed field and see at once how it moves the result.

# Ohm's Law

**Category:** Basic. The simplest calculator in the program, and the one to start with.

## What it does

You know two of four quantities, voltage $U$, current $I$, resistance $R$ and power $P$, and the page calculates the other two as well as the conductance $G$. It implements the six combinations of two known quantities:

| You know | Formulas used                                                |
|----------|--------------------------------------------------------------|
| $U$, $I$ | $R = U / I$,  $P = U \cdot I$                                 |
| $U$, $R$ | $I = U / R$,  $P = U \cdot I$                                 |
| $U$, $P$ | $I = P / U$,  $R = U / I$                                     |
| $I$, $R$ | $U = I \cdot R$,  $P = U \cdot I$                             |
| $I$, $P$ | $U = P / I$,  $R = U / I$                                     |
| $R$, $P$ | $U = \sqrt{P \cdot R}$,  $I = \sqrt{P / R}$                   |

## How to use it

1. Enter **exactly two** quantities. The other two fields turn grey.
2. Read the results table: it contains all five quantities $U$, $I$, $R$, $G$ and $P$ with their nominal values and spread.

All four inputs are positive numbers and accept tolerances, for example `4.7k 5%`. If you fill in three fields the page warns *Given 3 — exactly 2 required*; with one field it asks for one more.

## Worked example: a current-limiting resistor

An LED needs 10 mA and drops 2 V. It is powered from 5 V. The resistor must drop the remaining 3 V while carrying 10 mA, so enter $U = 3$ and $I = 10\text{m}$. The page returns $R = 300\ \Omega$ and $P = 30\ \text{mW}$. The nearest E24 value is 300 Ω itself; a 0.1 W part is plenty (power ratings are normally chosen at least 2× above the dissipation).

Now try $R = 330$ with `5%` and $U = 3$: the current comes out between 8.66 mA and 9.57 mA, and the table shows the worst-case spread, so you can check that the LED is neither too dim nor overdriven.

> **Remember.** The ratio of the worst-case power to the rating matters more than the nominal figure: always read the **Max** column of the $P$ row when you select a part.

# Voltage Divider

**Category:** Basic. A pair of resistors that produces a lower voltage from a higher one: the most common circuit in electronics.

## The circuit

![Two-resistor voltage divider](fig:divider2)

The same current $I$ flows through both resistors, so

$$I = \frac{V_{in}}{R_1 + R_2}, \qquad V_{out} = I \cdot R_2 = V_{in} \cdot \frac{R_2}{R_1 + R_2}$$

The ratio $R_2 / (R_1 + R_2)$ is always between 0 and 1, so a divider can only *reduce* a voltage. The output is a fraction of the input, and the fraction depends only on the *ratio* of the resistors, not on their absolute values. That is why you have a free choice of scale, which you use to trade accuracy and power consumption (see below).

## Two-resistor mode

Choose **2 resistors** in the *Configuration* list and provide **any three** of the four quantities $V_{in}$, $V_{out}$, $R_1$, $R_2$. The fourth is computed:

| You know                | Computed                                              |
|-------------------------|-------------------------------------------------------|
| $V_{in}$, $R_1$, $R_2$  | $V_{out} = V_{in} R_2 / (R_1 + R_2)$                  |
| $V_{in}$, $V_{out}$, $R_1$ | $R_2 = R_1 V_{out} / (V_{in} - V_{out})$           |
| $V_{in}$, $V_{out}$, $R_2$ | $R_1 = R_2 (V_{in} - V_{out}) / V_{out}$           |
| $V_{out}$, $R_1$, $R_2$ | $V_{in} = V_{out} (R_1 + R_2) / R_2$                  |

If $V_{out} \ge V_{in}$, the page reports *Vout must be less than Vin*.

## Three-resistor mode

![Three-resistor voltage divider with two taps](fig:divider3)

With three resistors the divider has two taps:

$$V_{out1} = V_{in} \cdot \frac{R_2 + R_3}{R_1 + R_2 + R_3}, \qquad V_{out2} = V_{in} \cdot \frac{R_3}{R_1 + R_2 + R_3}$$

Choose **3 resistors** and provide **any four** of the six quantities. The page solves the remaining two. Typical uses are reference ladders for comparators (upper and lower thresholds), battery-voltage monitors and bias networks. The ordering rule $V_{in} > V_{out1} > V_{out2} > 0$ always holds and is checked.

## Choosing the resistor values

Any ratio can be built from infinitely many pairs of resistors. The divider always draws $V_{in}^2 / (R_1 + R_2)$ from the source: for a 12 V rail across $2\ \text{k}\Omega$ that is 72 mW, wasted continuously; across $200\ \text{k}\Omega$ it is only 0.72 mW. Battery circuits need large values; for precision circuits 1–100 kΩ is a typical choice.

## Tolerance of the ratio

Resistor tolerances shift the output. If $R_1$ and $R_2$ both have tolerance $t$, the worst case occurs when one is at the top and the other at the bottom of its range, and the relative error of the ratio $k = R_2 / (R_1 + R_2)$ is at most

$$\frac{\Delta k}{k} \approx 2\,t\,(1 - k)$$

For $k = 0.5$ and 1 % resistors the output can be off by 1 %; for a divider with $k = 0.1$ it can be off by 1.8 %. The page applies tolerances automatically: add `1%` to the resistor fields and compare the **Min** and **Max** columns of $V_{out}$.

## Worked example: 12 V to 3.3 V

You need 3.3 V from a 12 V rail and decide on $R_2 = 10\ \text{k}\Omega$.

1. **Ideal value.** Enter $V_{in} = 12$, $V_{out} = 3.3$ and $R_2 = 10\text{k}$. The page computes $R_1 = R_2 (V_{in} - V_{out}) / V_{out} = 10\ \text{k}\Omega \cdot 8.7 / 3.3 = 26.36\ \text{k}\Omega$, which is not a standard value.
2. **Standard value.** In the **Resistor Solver** enter `26.3636k` and select E96: the nearest value is 26.1 kΩ. Then $V_{out} = 12 \cdot 10 / 36.1 = 3.324$ V, 0.7 % above the goal.
3. **Tolerance.** Back in the divider, enter $R_1 = 26.1\text{k}\ 1\%$ and $R_2 = 10\text{k}\ 1\%$: the output lies between 3.276 V and 3.372 V (−1.44 %, +1.45 %), close to the estimate $2t(1-k) = 2 \cdot 1\,\% \cdot 0.72 = 1.44\,\%$.
4. **Current and power.** $I = 12 / 36.1\ \text{k}\Omega = 0.33$ mA; the divider dissipates $12^2 / 36.1\ \text{k}\Omega = 4$ mW, nothing for a 0.1 W part.

# Resistor Solver

**Category:** Basic. Finds the best realisation of an arbitrary resistance using standard values.

## What it does

You enter a **target resistance** and choose an **E-series** (see *Foundations*). In addition you may type your own **custom pool**: the list of resistors you really have in your drawer, separated by commas, semicolons or spaces, with SI prefixes, for example `10k, 2k, 100`. Invalid or duplicate entries are ignored.

For both the chosen series and the custom pool the page shows three realisations:

| Row            | Meaning                                                              |
|----------------|----------------------------------------------------------------------|
| Single (1×)    | the closest single resistor                                          |
| Series (2×)    | the pair $R_a + R_b$ whose sum is closest to the target              |
| Parallel (2×)  | the pair $R_a \parallel R_b$ whose combination is closest            |

Each cell shows the values and the resulting resistance, and below it the **deviation** from the target in percent. A green **✓** means the error is within the typical tolerance of the series (20 % for E6, 10 % for E12, 5 % for E24, 2 % for E48, 1 % for E96); a yellow **⚠** means it is not. For the custom pool the same threshold of the selected series is used as a yardstick.

## How combinations help

A combination gets closer to the target than any single standard value can. Two ideas are worth knowing:

- **Series trimming.** To reach 10.12 kΩ pick a large resistor just below the target and add a small one: 10 kΩ + 120 Ω gives exactly 10.12 kΩ. The small resistor trims the result and contributes little to the total tolerance.
- **Parallel trimming.** Putting a much larger resistor in parallel *lowers* the value slightly: $10\ \text{k}\Omega \parallel 1\ \text{M}\Omega = 9.9\ \text{k}\Omega$. This is useful when only values slightly above the target are available.

For resistors of the same type and tolerance the relative tolerance of a series or parallel combination is no worse than that of its parts, and the contribution of a small trimming resistor is proportionally smaller. Combinations cost board space and one more soldering step, so use them only when accuracy requires it, for example for the feedback dividers of regulators or for precision gains.

## Worked example

Default values: target 10.12 kΩ, E24. The results: the closest single E24 value is 10 kΩ (−1.19 %, ✓), the best series pair is 10 kΩ + 120 Ω (+0.00 %), and the best parallel pair 11 kΩ ∥ 130 kΩ gives 10.14 kΩ (+0.22 %). With the custom pool `10k, 2k, 100` a single 10 kΩ again has the error −1.19 %, and the best pair is 10 kΩ + 100 Ω = 10.1 kΩ (−0.20 %).

# AWG Table

**Category:** Basic. A reference for copper wire of every common American Wire Gauge, from 40 AWG to 0000 (4/0), with the voltage drop, dissipated power and temperature rise for *your* current and length.

## Wire gauge

**AWG** (American Wire Gauge) numbers wires by how many times they were drawn through successive dies: the *larger* the number, the *thinner* the wire. The diameter follows a geometric law between 36 AWG (0.127 mm) and 0000 AWG (11.68 mm):

$$d_n = 0.127\ \text{mm} \cdot 92^{\,(36 - n)/39}$$

Useful rules of thumb that follow from it:

- every **6 gauges** the diameter doubles (or halves);
- every **3 gauges** the cross-section, and so the conductance, doubles;
- every **10 gauges** the cross-section changes by a factor of 10.

The cross-section is $A = \pi d^2 / 4$. The table uses the nominal diameters of the standard **ASTM B258**. Outside North America conductors are specified by their cross-section in mm² (**IEC 60228**: 0.5, 0.75, 1, 1.5, 2.5 mm² and so on): 0.5 mm² is close to 20 AWG, 1.5 mm² to 16 AWG, 2.5 mm² to 14 AWG.

## Resistance, drop and heating

The resistance of a uniform conductor of length $L$ and cross-section $A$ made of a material with resistivity $\rho$ is

$$R = \rho \frac{L}{A}$$

The calculator uses $\rho = 1.72 \cdot 10^{-8}\ \Omega\text{m}$, the resistivity of annealed copper at 20 °C (the international annealed copper standard, **IEC 60028**, defines $1/58\ \Omega\,\text{mm}^2/\text{m} = 1.7241 \cdot 10^{-8}\ \Omega\text{m}$). If the conductor is a bundle of $n$ strands in parallel, the cross-section is multiplied by $n$ and the resistance drops by the same factor. A current $I$ then causes

$$V_{drop} = I \cdot R, \qquad P = I^2 \cdot R, \qquad \Delta T = P \cdot R_{th}$$

where $R_{th}$ is the **thermal resistance** in K/W: how many kelvins the wire warms up for every watt dissipated in it, which depends on insulation, mounting and airflow. The temperature rise is relative to the surroundings.

## How to use it

1. Enter the current $I$ (A), the length $L$ (m) and the thermal resistance $R_{th}$ (K/W) of the wire. The defaults are 1 A, 1 m and 50 K/W. All three are plain positive numbers without tolerance.
2. The table lists every gauge with its diameter $D$, cross-section $A$, resistance $R$, voltage drop $V_{drop}$, power $P$ and temperature rise $\Delta T$.
3. If you use a stranded bundle, change the **n** value in the last column of the row: the resistance, drop, power and heating of that row are recalculated for $n$ parallel wires.

Look down the column of the quantity you care about, for example the drop at the end of a long supply line, and pick the thinnest wire that still meets the limit.

## Worked example

A 2 m supply cable carries 1 A to a sensor. For 24 AWG ($d = 0.511$ mm, $A = 0.205\ \text{mm}^2$) the resistance of 1 m is $0.084\ \Omega$. Enter $L = 4$ (the current flows out *and back*, so the loop is 4 m): $R = 0.336\ \Omega$, $V_{drop} = 0.336$ V, $P = 0.336$ W. With $R_{th} = 50$ K/W the wire warms by about 17 K, which is acceptable. The same loop of 30 AWG would drop 1.35 V and warm up by 68 K, which is not.


# Current Shape

**Category:** Basic. Computes the **RMS** and the **average** value of a periodic current pulse, and its duty cycle, for six common waveforms.

## Why RMS and average

The current in power electronics is rarely constant: it is a train of pulses. Different effects depend on different properties of the waveform:

- **Heating** in resistive parts (wires, MOSFET channel, inductor winding, capacitor ESR) is $P = I_{rms}^2 R$. The **RMS** (root mean square) current is *defined* as the DC current that would dissipate the same power:

$$I_{rms} = \sqrt{\frac{1}{T} \int_0^T i^2(t)\, dt}$$

- **The average value** $I_{avg}$ determines what a DC meter reads, the charge delivered to a battery, and the loss in a device with a fixed voltage drop such as a diode: $P = V_f \cdot I_{avg}$.
- **The peak value** determines whether an inductor saturates and whether a fuse or semiconductor limit is exceeded.

The **duty cycle** $D = t_{on} / T$ is the fraction of the period in which the current flows. Two more figures of merit are the **crest factor** $I_p / I_{rms}$ and the **form factor** $I_{rms} / I_{avg}$. For a constant current both equal 1; the sharper the pulses, the larger they get.

## The six waveforms

Choose the waveform in the *Waveform* list. The preview on the right redraws as you type; for empty fields it shows a sample shape so you always see what is being calculated. In the plot the blue line is $I_{rms}$ and the orange line is $I_{avg}$; dimension arrows show the times and the peak current.

### Half-sine

![Half-sine pulse](fig:wave_half_sine)

Inputs: peak $I_p$, pulse width $t_{on}$, period $T$. This is the shape of a rectified sine and of the current in resonant converters.

$$D = \frac{t_{on}}{T}, \qquad I_{rms} = I_p \sqrt{\frac{D}{2}}, \qquad I_{avg} = \frac{2}{\pi} I_p D$$

### Rectangular

![Rectangular pulse](fig:wave_rect)

Inputs: peak $I_p$, $t_{on}$, $T$. The classic PWM current.

$$I_{rms} = I_p \sqrt{D}, \qquad I_{avg} = I_p D$$

The square root is the key observation: a pulse with 25 % duty cycle has an RMS of **50 %** of its peak but an average of only 25 %.

### Trapezoidal

![Trapezoidal pulse](fig:wave_trapezoid)

Inputs: current at the start $I_{p1}$ and at the end $I_{p2}$ of the pulse, $t_{on}$ and $T$. The current changes linearly during the pulse, as in the inductor current of a buck converter in continuous mode.

$$I_{rms} = \sqrt{D \cdot \frac{I_{p1}^2 + I_{p1} I_{p2} + I_{p2}^2}{3}}, \qquad I_{avg} = D \cdot \frac{I_{p1} + I_{p2}}{2}$$

For $I_{p1} = I_{p2}$ this reduces to the rectangular pulse, and for $I_{p1} = 0$ to a sawtooth.

### Triangular

![Triangular pulse](fig:wave_triangular)

Inputs: peak $I_{max}$ and the times $t_1$ (rise), $t_2$ (fall), $t_3$ (pause) and $T$. **Enter any three of $t_1$, $t_2$, $t_3$ and $T$:** the fourth field becomes grey and shows the value that follows from $T = t_1 + t_2 + t_3$.

$$D = \frac{t_1 + t_2}{T}, \qquad I_{rms} = I_{max}\sqrt{\frac{D}{3}}, \qquad I_{avg} = \frac{I_{max}}{2}\, D$$

### Triangular with offset

![Triangular pulse with offset](fig:wave_offset)

The current rises from a floor $I_{min}$ to $I_{max}$ in $t_1$ and falls back in $t_2$. This is the shape of an inductor ripple current in a converter that does not go to zero. Times are entered as for the triangular waveform; $I_{min}$ must be positive and below $I_{max}$.

$$I_{rms} = \sqrt{D \cdot \frac{I_{min}^2 + I_{min} I_{max} + I_{max}^2}{3}}, \qquad I_{avg} = D \cdot \frac{I_{min} + I_{max}}{2}$$

### Bipolar triangular

![Bipolar triangular pulse](fig:wave_bipolar)

A negative triangle of peak $I_{min}$ (a *negative* number) lasting $t_1$, followed by a positive triangle of peak $I_{max}$ lasting $t_2$, as in the winding current of a transformer in a push-pull stage. The positive and negative lobes partly cancel in the average, but both heat the winding:

$$I_{avg} = \frac{I_{min}\, t_1 + I_{max}\, t_2}{2\,T}, \qquad I_{rms} = \sqrt{\frac{I_{min}^2\, t_1 + I_{max}^2\, t_2}{3\,T}}$$

## Results

The Results table gives $I_{rms}$, $I_{avg}$ and the duty cycle. The inputs are exact numbers (tolerances are not accepted). The page checks that the geometry is possible: a pulse cannot be longer than the period (*pulse width must not exceed period*), times must be positive, and the peak current of the pulse waveforms cannot be negative.

## Worked example

A MOSFET switches 2 A pulses of 1 ms every 4 ms. Choose **Rectangular** and enter $I_p = 2$, $t_{on} = 1\text{m}$, $T = 4\text{m}$. The page gives $D = 0.25$, $I_{rms} = 1$ A, $I_{avg} = 0.5$ A. With $R_{DS(on)} = 50\ \text{m}\Omega$ the conduction loss is $P = I_{rms}^2 R_{DS(on)} = 50$ mW, *not* the $0.2$ W you would get from the peak current and not the 12.5 mW from the average current. The same pulse through a diode with $V_f = 0.7$ V dissipates $0.7 \cdot 0.5 = 0.35$ W.

> **Rule.** Use $I_{rms}$ for anything resistive, $I_{avg}$ for anything with a fixed voltage drop (diodes, saturated transistors, batteries) and $I_p$ for saturation and absolute limits.


# Operational amplifiers

The *Analog* category holds five op-amp calculators: the **inverting** and **non-inverting** amplifiers, the **difference amplifier** and two **level shifters**. This chapter explains the op-amp itself and the two basic amplifiers; the next chapters build on it.

## The ideal op-amp

An operational amplifier is a differential amplifier with an enormous open-loop gain $A$ (typically $10^5$–$10^7$) between its non-inverting input (+) and its inverting input (−):

$$V_{out} = A\,(V_+ - V_-)$$

An amplifier with such a gain is only useful inside a **negative feedback loop**: part of the output is returned to the inverting input, so that the loop forces $V_+ - V_- = V_{out}/A$ to be practically zero. Two assumptions then describe the op-amp:

1. **The inputs draw no current.**
2. **The voltage between the inputs is zero**: the *virtual short*. The feedback adjusts the output until this is true.

All formulas of the op-amp chapters follow from these two rules, Ohm's law and Kirchhoff's current law at the inverting input. The TI designs below keep the resistors in the range 1–100 kΩ.

## Inverting amplifier

![Inverting amplifier](fig:opamp_inv)

The non-inverting input is grounded, so by the virtual short the inverting input is also at 0 V, a *virtual ground*. KCL at that node: the input current $V_{in}/R_1$ cannot enter the op-amp, so it all flows through the feedback resistor $R_2$, and $V_{out} = -R_2 \cdot V_{in}/R_1$:

$$G = \frac{V_{out}}{V_{in}} = -\frac{R_2}{R_1}$$

The gain is negative: the output is the input flipped upside down. Its magnitude can be smaller or larger than 1. The input resistance of the stage is just $R_1$, so the source must be able to drive that load.

## Non-inverting amplifier

![Non-inverting amplifier](fig:opamp_noninv)

Here the signal goes straight to the + input. The feedback divider $R_1$ (from the output) and $R_2$ (to ground) returns the fraction $R_2 / (R_1 + R_2)$ of the output to the − input. The virtual short makes this fraction equal to $V_{in}$:

$$V_{in} = V_{out} \frac{R_2}{R_1 + R_2} \quad\Rightarrow\quad G = 1 + \frac{R_1}{R_2}$$

The gain is positive and **at least 1**. The input resistance is extremely high, because the signal only meets the op-amp's input, which makes this stage ideal for buffering weak sources. $R_1 = 0$ and $R_2 = \infty$ give the *voltage follower* with $G = 1$; the page requires $G > 1$ and does not cover it.

> **Resistor names.** On these two pages $R_1$ and $R_2$ are named as in most textbooks: $R_1$ is the input resistor of the inverting stage and the feedback resistor of the non-inverting stage. The chapters that follow use the names of the TI documents they rely on, where $R_1$ is always the feedback resistor.

## The Op-Amp pages

![Output of the two amplifiers against the input: the slope is the gain, negative for the inverting stage](fig:transfer)

The schematic itself is the form. Its fields are the **Vin** range (under the input), the **Vout** range (under the output), $R_1$ and $R_2$ (above the resistors) and the gain **G** (inside the amplifier). Fill in **enough** of them and the rest is calculated; the computed fields lock and their values appear in the results table.

| You enter                                        | You get                                         |
|--------------------------------------------------|-------------------------------------------------|
| $R_1$, $R_2$, Vin range                          | $G$, Vout range                                 |
| $G$, $R_1$ (or $R_2$), Vin range                 | the other resistor, Vout range                  |
| Vin range, Vout range, $R_1$                     | $G$, $R_2$                                      |
| Vout range, $G$                                  | Vin range                                       |
| a single value of Vin and of Vout                | $G$ (then add a resistor to get the other one)  |

Notes on the entry:

- A range is typed as `min; max`; a single number is one value (see *Ranges* in *Quick start*).
- For the **inverting** amplifier the gain is negative, so the *minimum* input gives the *maximum* output. The page pairs the ends itself: $V_{in}$ = `-1; 1` and $G = -5$ give $V_{out}$ = −5…5 V. A positive $G$ is refused (*G must be negative*).
- For the **non-inverting** amplifier $G$ must be greater than 1.
- Voltages and the gain are exact numbers. **Resistors accept tolerances**, for example `10k 1%`. Entering $R_1$, $R_2$ and $G$ together is allowed: the gain is then taken from the resistors.

The results table lists **Vin**, **Vout, ideal** (with nominal resistors) and, when there are tolerances, **Vout, with tolerances** (the lowest and the highest output the circuit can give), then $R_1$, $R_2$ and $G$ with their spread. The **Plot** tab shows $V_{out}(V_{in})$; with tolerances it adds the lines of the largest and the smallest gain, the band in which the real output lies.

## Tolerance of the gain

The gain depends on the resistor *ratio*, so resistor tolerance goes straight into the accuracy of the stage. In the inverting amplifier the gain is largest when the feedback resistor is at its maximum and the input resistor at its minimum:

$$\lvert G \rvert_{max} = \frac{R_2 (1 + t)}{R_1 (1 - t)} \approx \lvert G \rvert\,(1 + 2t)$$

For 1 % resistors the gain is uncertain by about ±2 %; with 0.1 % parts by ±0.2 %. In the non-inverting stage the error is smaller, because $G = 1 + R_1/R_2$ contains an exact 1: for $G = 11$ the relative error is about $2t \cdot 10/11$.

## Worked example: scaling a sensor

A sensor gives −1 V to +1 V and the ADC accepts −5 V to +5 V.

1. **Gain.** Open **Op-Amp (inverting)** and enter $V_{in}$ = `-1; 1` and $V_{out}$ = `-5; 5`. The page finds $G = -5$.
2. **Resistors.** Add $R_1$ = `10k`: the page returns $R_2 = 50\ \text{k}\Omega$. That is not a standard value; the nearest E96 value is 49.9 kΩ.
3. **Real parts.** Clear $V_{out}$ and enter $R_1$ = `10k 1%`, $R_2$ = `49.9k 1%`. Now $G = -4.99$ nominally and between −5.091 and −4.891 (±2 %) in the worst case, and the output reaches ±5.091 V. If the ADC must not see more than ±5 V, lower the gain slightly or use 0.1 % resistors.

If you need a positive gain and a high input resistance, choose **Op-Amp (non-inverting)**: with $R_1 = 50\ \text{k}\Omega$ and $R_2 = 10\ \text{k}\Omega$ ($G = 6$) an input of 0…2 V becomes 0…12 V.

# Difference amplifier

**Category:** Analog. **Source:** TI, *Difference amplifier (subtractor) circuit*, [SBOA274](https://www.ti.com/lit/SBOA274).

A difference amplifier amplifies the **difference** of two voltages and ignores what they have in common. It reads the voltage across a current-sense resistor that sits away from ground, the output of a bridge sensor, or a signal whose ground is not the ground of the ADC.

## The circuit

![Difference amplifier: R2 = R1 and R4 = R3](fig:differential)

$V_{i1}$ reaches the inverting input through $R_1$, with $R_3$ as the feedback; $V_{i2}$ reaches the non-inverting input through the divider $R_2$, $R_4$, whose lower end goes to the reference $V_{ref}$. By **superposition**:

- $V_{i1}$ alone (the + input held fixed) sees an inverting amplifier: $-V_{i1} \dfrac{R_3}{R_1}$.
- $V_{i2}$ and $V_{ref}$ set the + input to $V_+ = \dfrac{V_{i2} R_4 + V_{ref} R_2}{R_2 + R_4}$, which the stage amplifies like a non-inverting amplifier: $V_+ \left( 1 + \dfrac{R_3}{R_1} \right)$.

$$V_{out} = -V_{i1} \frac{R_3}{R_1} + \frac{V_{i2} R_4 + V_{ref} R_2}{R_2 + R_4} \left( 1 + \frac{R_3}{R_1} \right)$$

With **matched pairs** $R_2 = R_1$ and $R_4 = R_3$ this collapses to

$$V_{out} = (V_{i2} - V_{i1}) \cdot G + V_{ref}, \qquad G = \frac{R_3}{R_1}$$

What the two inputs have in common cancels, and $V_{ref}$ is added to the output unchanged. A reference at mid-supply moves a bipolar difference into the range of a single-supply ADC.

## Input resistance and the reference

- The two inputs load their sources differently: $V_{i1}$ sees $R_1$ (to a node that moves with the signal), $V_{i2}$ sees $R_2 + R_4$. Between the inputs the resistance is $R_1 + R_2$.
- **Source resistance adds to $R_1$ and $R_2$** and so changes the gain and the balance of the pairs. Drive the inputs from low-impedance sources, or buffer each input: that is the *instrumentation amplifier*.
- **The reference must be stiff.** Its source resistance adds to $R_4$. Use a reference IC or a buffered divider, not a bare resistor divider.

## The page

The fields sit on the schematic: the difference range **Vin diff** under the inputs, the **Vout** range under the output, **G** inside the amplifier, $R_1$ and $R_3$ above their resistors and **Vref** at the reference terminal. $R_2 = R_1$ and $R_4 = R_3$ are implied.

- **Any two of Vin diff, Vout and G** give the third ($V_{out} = V_{diff} \cdot G + V_{ref}$). $R_3$ together with $R_1$ also fixes the gain.
- One resistor and the gain give the other: $R_3 = G \cdot R_1$, or $R_1 = R_3 / G$.
- $V_{ref}$ is required. A single number in a range field means the range from 0 to it.
- $V_{ref}$, $R_1$ and $R_3$ accept tolerances. Each of the four resistors may then sit anywhere in its range, independently (a calculated resistor takes the tolerance of the other one), so the table shows the true worst case of a real board, mismatch included.

The results table lists $R_1 = R_2$, $R_3 = R_4$, Vin, Vout (ideal and with tolerances) and $G$. The plot shows the output against the difference, the level of $V_{ref}$ and, with tolerances, the band of the output.

## Worked example: the SBOA274 design

Goal: a difference of ±1.25 V becomes ±2.5 V, $V_{ref} = 0$.

1. **Gain.** Enter Vin diff = `-1.25; 1.25`, Vout = `-2.5; 2.5` and Vref = `0`. $G = 5 / 2.5 = 2$.
2. **Resistors.** Enter $R_1$ = `10k`: $R_3 = G \cdot R_1 = 20\ \text{k}\Omega$. Both are standard values.
3. **Tolerance.** Enter $R_1$ = `10k 1%`, $R_3$ = `20k 1%`: $G$ is 1.960…2.040 (±2 %) and the output reaches ±2.551 V. With 0.1 % resistors the gain error drops to ±0.2 %.

**Single supply.** For a 3.3 V ADC, shift the output to mid-supply: Vin diff = `-0.8; 0.8`, $G = 2$, $V_{ref} = 1.65$ V give $V_{out}$ = 0.05…3.25 V. With 0.1 % resistors the output stays within 46.7 mV…3.256 V.

# Level shifters

**Category:** Analog. Two calculators, **Level Shifter (inverting)** and **Level Shifter (non-inverting)**, design the classic interface between a sensor and an ADC: a range that does not fit the converter is scaled *and moved* onto its input range.

## Gain and offset

A level shifter maps the input range $V_{in,min} \dots V_{in,max}$ onto the output range $V_{out,min} \dots V_{out,max}$ with a straight line:

$$V_{out} = G \cdot V_{in} + V_{offset}, \qquad \lvert G \rvert = \frac{V_{out,max} - V_{out,min}}{V_{in,max} - V_{in,min}}$$

The gain comes from the two spans; the offset from any one pair of matching ends. A plain amplifier gives the right slope through zero; a **reference voltage** fed through a resistor moves the line up or down by the offset.

The two calculators follow two designs of TI's *Analog Engineer's Circuit* series:

| Calculator | Document | Signal enters | Reference enters |
|---|---|---|---|
| Level Shifter (inverting) | [SBOA261](https://www.ti.com/lit/SBOA261) | inverting input | inverting input |
| Level Shifter (non-inverting) | [SBOA262](https://www.ti.com/lit/SBOA262) | non-inverting input | inverting input |

In both the reference enters the **inverting** input. A reference there contributes with a minus sign: a positive $V_{ref}$ moves the output **down**, a negative one moves it up. In these chapters, as in the TI documents, $R_1$ is the feedback resistor.

## Inverting level shifter

![Inverting level shifter (SBOA261): Vin through R2 and Vref through R3 meet at the inverting input, R1 is the feedback](fig:shifter_inv)

The + input is grounded, so the − input is a virtual ground. KCL there: the currents from $V_{in}$ through $R_2$, from $V_{ref}$ through $R_3$ and from the output through $R_1$ add up to zero:

$$\frac{V_{in}}{R_2} + \frac{V_{ref}}{R_3} + \frac{V_{out}}{R_1} = 0 \quad\Rightarrow\quad V_{out} = -V_{in} \frac{R_1}{R_2} - V_{ref} \frac{R_1}{R_3}$$

The stage inverts: $V_{in,max}$ gives $V_{out,min}$. The design follows from the two terms:

1. **Gain.** $\lvert G \rvert = R_1/R_2$ from the spans.
2. **Resistors of the gain.** Choose $R_1$ (1–100 kΩ), then $R_2 = R_1 / \lvert G \rvert$.
3. **Offset.** At $V_{in,max}$ the output must be $V_{out,min}$: $V_{out,min} = -V_{in,max} \lvert G \rvert - V_{ref} R_1/R_3$, so

$$\frac{R_1}{R_3} = -\frac{V_{out,min} + \lvert G \rvert\, V_{in,max}}{V_{ref}}$$

4. **Reference resistor.** $R_3 = R_1 / (R_1/R_3)$. This must be positive: $V_{ref}$ needs the sign opposite to $C = V_{out,min} + \lvert G \rvert V_{in,max}$. If $C = 0$ no offset is needed and $R_3$ is left out.
5. **Standard values.** Round $R_2$ and $R_3$ and check the real output.

![The gain alone gives the right slope in the wrong place; the reference moves the line into the window of the two ranges (SBOA261 example)](fig:shift_inv)

**Practical points.** The input resistance is $R_2$: the source must drive it. The reference drives $R_3$ and must be stiff.

## Worked example: the SBOA261 design

Goal: a sensor gives −5 V…−1 V, the ADC takes 0.05 V…3.3 V, the reference is 5 V. Enter Vin = `-5; -1`, Vout = `0.05; 3.3`, Vref = `5`, $R_1$ = `845`.

1. $\lvert G \rvert = \dfrac{3.3 - 0.05}{-1 - (-5)} = \dfrac{3.25}{4} = 0.8125$ (the page shows $G = -0.8125$).
2. $R_2 = 845\ \Omega / 0.8125 = 1.040\ \text{k}\Omega$, an E192 value.
3. $\dfrac{R_1}{R_3} = -\dfrac{0.05 + 0.8125 \cdot (-1)}{5} = \dfrac{0.7625}{5} = 0.1525$.
4. $R_3 = 845\ \Omega / 0.1525 = 5.541\ \text{k}\Omega$; the nearest E192 value is 5.56 kΩ.
5. Enter $R_3$ = `5.56k`: the output becomes 52.6 mV…3.3026 V, 2.6 mV above the goal at both ends, less than 0.1 % of the span.
6. With 1 % resistors (`845 1%`, `1.04k 1%`, `5.56k 1%`) the output spreads over 36.5 mV…3.385 V and the gain over ±2 %: slightly beyond the 0.05…3.3 V asked for. 0.1 % resistors keep it within about 0.2 %.

## Non-inverting level shifter

![Non-inverting level shifter (SBOA262): Vin through the divider R3, R4 to the + input, Vref through R2 to the − input, R1 is the feedback](fig:shifter_ni)

The input divider sets the + input to $V_+ = V_{in} \cdot K$ with $K = R_4/(R_3 + R_4)$. KCL at the − input, which equals $V_+$, with $V_{ref}$ through $R_2$ and the output through $R_1$:

$$\frac{V_{ref} - V_+}{R_2} + \frac{V_{out} - V_+}{R_1} = 0 \quad\Rightarrow\quad V_{out} = V_{in} \frac{R_4}{R_3 + R_4} \left( 1 + \frac{R_1}{R_2} \right) - V_{ref} \frac{R_1}{R_2}$$

The slope is $G = K (1 + R_1/R_2)$, and the offset is $-V_{ref} R_1/R_2$. The design:

1. **Gain.** $G$ from the spans; here $V_{in,min}$ gives $V_{out,min}$.
2. **Offset.** At $V_{in,min}$: $V_{out,min} = G V_{in,min} - V_{ref} R_1/R_2$, so with $D = G V_{in,min} - V_{out,min}$

$$\frac{R_1}{R_2} = \frac{D}{V_{ref}}, \qquad R_2 = R_1 \frac{V_{ref}}{D}$$

  $V_{ref}$ must have the sign of $D$. If $D = 0$ there is nothing to shift: use a plain non-inverting amplifier.
3. **Divider.** $K = G / (1 + R_1/R_2)$; choose $R_4$, then $R_3 = R_4 (1/K - 1)$.
4. **The limit.** A divider can only attenuate, $K < 1$, so $1 + R_1/R_2 > G$. With $R_1/R_2 = D/V_{ref}$ this limits the reference: for $G > 1$

$$\lvert V_{ref} \rvert < \frac{\lvert D \rvert}{G - 1}$$

  Above it the page reports that $R_3$ would not be positive and tells you the limit.

![The same idea for the non-inverting stage (SBOA262 example)](fig:shift_ni)

**Practical points.** The input resistance is $R_3 + R_4$. The reference drives $R_2$ and must be stiff.

## Worked example: the SBOA262 design

Goal: 2 V…5 V into 0.05 V…4.95 V with a 2.5 V reference, $R_1 = R_4 = 1\ \text{k}\Omega$. Enter Vin = `2; 5`, Vout = `0.05; 4.95`, Vref = `2.5`, $R_1$ = `1k`, $R_4$ = `1k`.

1. $G = \dfrac{4.95 - 0.05}{5 - 2} = 1.6333$.
2. $D = 1.6333 \cdot 2 - 0.05 = 3.2167$ V; $R_1/R_2 = 3.2167 / 2.5 = 1.2867$; $R_2 = 1\ \text{k}\Omega / 1.2867 = 777.2\ \Omega$ → E192: 777 Ω.
3. $K = 1.6333 / 2.2867 = 0.7143$; $R_3 = 1\ \text{k}\Omega \cdot (1/0.7143 - 1) = 400\ \Omega$ → E96: 402 Ω.
4. Enter $R_2$ = `777`, $R_3$ = `402`: the output is 45.0 mV…4.939 V and the gain 1.6312.
5. The reference may go up to $3.2167 / 0.6333 = 5.08$ V; 2.5 V is about half of that, a comfortable choice.
6. With 1 % resistors everywhere the output spreads over −1.9 mV…5.014 V, well beyond the 0.05…4.95 V asked for: 1 % parts are not good enough here, use 0.1 % resistors.

## Choosing the reference

The reference sets the offset, so it must be **accurate**, **stable** and **stiff** (low output impedance):

- A **reference IC** (series references of 1.25, 2.048, 2.5, 3.0, 3.3, 4.096, 5 V and more) gives a stable, low-impedance voltage.
- The **ADC reference** itself makes the measurement *ratiometric*: the reference's error then cancels in the reading.
- A bare resistor divider is not stiff: its source resistance adds to $R_3$ (inverting) or $R_2$ (non-inverting). Buffer it with a follower.

**When the Vref field is empty**, the page chooses a value from the two ranges:

- If the given resistors already fix it (inverting: $R_3$ with $R_1$ or $R_2$; non-inverting: $R_1$ with $R_2$), the page computes the **required** reference: $V_{ref} = -C R_3/R_1$, or $V_{ref} = D R_2/R_1$.
- Otherwise it **recommends** one from the standard values 1.024, 1.25, 2.048, 2.5, 3.0, 3.3, 4.096, 5 and 10 V, with the sign the offset needs. For the inverting stage it takes the value nearest to $\lvert C \rvert$, so that $R_3$ comes out close to $R_1$. For the non-inverting stage it takes the value nearest to half of the limit $\lvert D \rvert/(G - 1)$, safely below it.
- If no offset is needed, the inverting page says so and leaves $R_3$ open.

The value appears as the grey hint of the Vref field and as a row of the results table. Type it, or your own, into the field. For the SBOA261 example the recommendation is 1.024 V with $R_3 = 1.135\ \text{k}\Omega$; TI uses the 5 V supply as the reference, which works too, with a larger $R_3$. For SBOA262 the recommendation is the 2.5 V of the document.

## The Level Shifter pages

The fields sit on the schematic: the **Vin** and **Vout** ranges, **G**, **Vref** and the resistors.

- **Both ranges** with $V_{ref}$ (given or chosen) fix the design. The inverting page then needs **one resistor**, $R_1$, $R_2$ or $R_3$, and computes the others. The non-inverting page needs **one resistor of each pair**: $R_1$ or $R_2$, and $R_3$ or $R_4$.
- **One range** is enough when the resistors fix the rest. Inverting: $R_3$ and two of $R_1$, $R_2$, $G$. Non-inverting: $R_1$, $R_2$ and either $G$ or $R_3$ with $R_4$.
- A typed resistor is used as typed: enter standard values into the computed fields to see the real output.
- $V_{ref}$ and the resistors accept tolerances; a computed resistor takes the tolerance of the given one it follows from.

The results table lists the resistors, $V_{ref}$ (given, required or recommended), Vin, Vout (ideal and with tolerances) and $G$. The plot shows the **required** line, the line **with these resistors**, the level of $V_{ref}$ and, with tolerances, the band of the output.

The page shows a red message when the ranges cannot be mapped: a reference of the wrong sign (with the value of the right sign to use), a reference above the limit of the non-inverting stage, ranges with the lower end above the upper one.

# Reference

## SI prefixes accepted in the input

The prefixes are those of the International System of Units (BIPM, *The International System of Units*, the SI Brochure). The input uses `u` for micro (µ).

| Symbol | Name  | Factor      | Example           |
|--------|-------|-------------|-------------------|
| `G`    | giga  | $10^{9}$    | `1G` = 1 000 000 000 |
| `M`    | mega  | $10^{6}$    | `2.2M`            |
| `k`    | kilo  | $10^{3}$    | `4.7k`            |
| (none) | —     | $1$         | `100`             |
| `m`    | milli | $10^{-3}$   | `500m`            |
| `u`    | micro | $10^{-6}$   | `10u`             |
| `n`    | nano  | $10^{-9}$   | `100n`            |
| `p`    | pico  | $10^{-12}$  | `22p`             |

Results are printed with the nearest engineering prefix (multiples of 1000), with the number of decimals set in Settings.

## Formula cheat sheet

| Topic                     | Formula                                                          |
|---------------------------|------------------------------------------------------------------|
| Ohm's law                 | $U = I R$                                                        |
| Power                     | $P = U I = I^2 R = U^2 / R$                                      |
| Series / parallel         | $R = R_1 + R_2$ / $R = R_1 R_2 / (R_1 + R_2)$                    |
| Voltage divider           | $V_{out} = V_{in} R_2 / (R_1 + R_2)$                             |
| Divider ratio tolerance   | $\Delta k / k \approx 2t(1 - k)$                                 |
| Wire resistance           | $R = \rho L / A$,  $\rho_{Cu} = 1.72 \cdot 10^{-8}\ \Omega\text{m}$ |
| AWG diameter              | $d = 0.127\ \text{mm} \cdot 92^{(36 - n)/39}$                    |
| Temperature rise          | $\Delta T = P R_{th}$                                            |
| Duty cycle                | $D = t_{on} / T$                                                 |
| RMS of a rectangle        | $I_p \sqrt{D}$                                                   |
| RMS of a half-sine        | $I_p \sqrt{D/2}$                                                 |
| RMS of a triangle         | $I_p \sqrt{D/3}$                                                 |
| Inverting amplifier       | $G = -R_2 / R_1$                                                 |
| Non-inverting amplifier   | $G = 1 + R_1 / R_2$                                              |
| Difference amplifier      | $V_{out} = (V_{i2} - V_{i1}) R_3/R_1 + V_{ref}$                  |
| Inverting level shifter   | $V_{out} = -V_{in} R_1/R_2 - V_{ref} R_1/R_3$                    |
| Non-inverting level shifter | $V_{out} = V_{in} \frac{R_4}{R_3 + R_4}(1 + \frac{R_1}{R_2}) - V_{ref} \frac{R_1}{R_2}$ |

## Standards

| Standard     | Subject                                                                 |
|--------------|-------------------------------------------------------------------------|
| IEC 60063    | Preferred number series for resistors and capacitors: E6 … E192         |
| IEC 60062    | Marking codes for resistors and capacitors (colour bands, letter codes) |
| IEC 60028    | International standard of resistance for copper (annealed copper)      |
| IEC 60228    | Conductors of insulated cables: metric cross-sections in mm²            |
| ASTM B258    | Nominal diameters and cross-sections of AWG sizes of round wire         |
| SI Brochure  | BIPM, *The International System of Units*: units and prefixes           |

## Application notes

The op-amp chapters follow these documents; they are free to download.

- Texas Instruments, *Inverting op amp with inverting positive reference voltage circuit*, [SBOA261](https://www.ti.com/lit/SBOA261). The design of **Level Shifter (inverting)**.
- Texas Instruments, *Non-inverting op amp with inverting positive reference voltage circuit*, [SBOA262](https://www.ti.com/lit/SBOA262). The design of **Level Shifter (non-inverting)**.
- Texas Instruments, *Difference amplifier (subtractor) circuit*, [SBOA274](https://www.ti.com/lit/SBOA274). The design of **Op-Amp (differential)**.
- Texas Instruments, B. Carter, R. Mancini, *Op Amps for Everyone*, [SLOD006](https://www.ti.com/lit/SLOD006): a complete design guide to op-amp circuits.

## Glossary

- **Nominal value**: the value a component or a result has with no deviation; the *Nom* column.
- **Tolerance**: the allowed deviation of a component from its nominal value, in percent (`10k 1%`).
- **Worst case**: the most unfavourable combination of the tolerances of all components; the *Min* and *Max* columns.
- **Δ, %**: how far Min and Max lie from the nominal value, in percent.
- **Interval arithmetic**: calculating with ranges (nominal, minimum, maximum) so that the result contains every possible outcome.
- **Corner analysis**: finding the worst case by evaluating a circuit at every combination of the minimum and maximum of its inputs.
- **SI prefix**: the letter after a number that scales it by a power of 1000: `k`, `M`, `m`, `u`, `n`…
- **Computed field**: a field the program fills from the others; it is locked until you clear one of your inputs.
- **Range**: the span of a signal, typed as `min; max`.
- **Kirchhoff's laws**: the currents into a node add up to zero (KCL); the voltages around a loop add up to zero (KVL).
- **Superposition**: in a linear circuit the response to several sources is the sum of the responses to each source alone.
- **E-series**: the preferred values of components, E6 to E192 (IEC 60063).
- **AWG**: American Wire Gauge, the wire size number; a larger number is a thinner wire.
- **Thermal resistance**: the temperature rise per watt of dissipated power, in K/W.
- **RMS**: root mean square, the effective value of a changing quantity; for a current, the DC value with the same heating effect.
- **Duty cycle**: the fraction of the period in which a pulse is on.
- **Crest factor / form factor**: peak over RMS, and RMS over average, of a waveform.
- **Virtual short / virtual ground**: the condition in a feedback loop that makes the two op-amp inputs equal, or the inverting input equal to ground.
- **Gain (G)**: the ratio of output to input change, in V/V; negative for an inverting stage.
- **Offset**: the shift a level shifter adds to the amplified signal, set by the reference.
- **Level shifter**: an amplifier with a reference that maps one voltage range onto another, with gain and offset.
- **Reference (Vref)**: an accurate, stable, low-impedance voltage that sets an offset. On the level shifter pages it is *given*, *required* (fixed by the given resistors) or *recommended* (chosen by the program).
- **Ratiometric**: a measurement whose result depends on a ratio of voltages, so that the error of a shared reference cancels.

## Messages you may see

| Message                                    | What to do                                                       |
|--------------------------------------------|------------------------------------------------------------------|
| Fix errors in the fields                   | a field has a red border; read its message (hover on a schematic)|
| Provide 3 of 4 quantities                  | fill in more fields (the number given is shown)                  |
| Given 3 — exactly 2 required               | clear a field: the calculator takes exactly this many inputs     |
| Vout1 must be less than Vin                | the voltages must be in the order of the divider                 |
| G must be negative / greater than 1        | the sign or size of the gain does not suit the amplifier type    |
| Give Vin and Vout as "min; max" …          | the lower end of a range must come first                         |
| Fill in two of: Vindiff, Vout, G           | the difference amplifier needs two of the three                  |
| Fill in Vref, or both Vin and Vout …       | give the reference, or both ranges to have it recommended        |
| Vref … has the wrong sign for this offset  | use a reference of the opposite sign (the value is suggested)    |
| R3 would not be positive …                 | the reference is too large for the non-inverting shifter: lower it below the limit shown |
| Vin min·G equals Vout min …                | no offset is needed: use Op-Amp (non-inverting)                  |
| pulse width must not exceed period         | $t_{on}$ is longer than $T$                                      |
| t1 + t2 + t3 must equal the period T       | clear one of the four times                                      |

## Further reading

- Texas Instruments, *Analog Engineer's Circuit Cookbook: Amplifiers*: the collection of circuits the op-amp pages come from.
- P. Horowitz, W. Hill, *The Art of Electronics*: the classic practical course.
- A. Sedra, K. Smith, *Microelectronic Circuits*: a thorough university text on amplifiers.
- B. Carter, R. Mancini, *Op Amps for Everyone*: TI's design guide (see *Application notes*).
- The datasheet of your component: always the last word on limits and tolerances.

# About

| Field     | Value                                                    |
|-----------|----------------------------------------------------------|
| Name      | ECW — Engineers' Circuit Workbench                       |
| Motto     | Fast calculations. Practical engineering.                |
| Version   | 0.2.0                                                    |
| License   | MIT                                                      |
| Platform  | Tauri 2, Rust core, Preact interface                     |
| Source    | [github.com/zvladimir/ecw](https://github.com/zvladimir/ecw) |
| Support   | [boosty.to/desialsi](https://boosty.to/desialsi)         |

ECW is free software. Found a wrong result? Please report it on GitHub with the values you entered, what you expected and the source of the formula.

© 2024–2026 Zibtsev Vladimir
