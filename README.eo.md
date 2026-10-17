<p align="center"><img src="ui/img/logo.svg" width="96" height="96" alt="Emblemo de ECW"></p>

# ECW — Engineers' Circuit Workbench

[English](README.md) · [Русский](README.ru.md) · **Esperanto**

*Rapidaj kalkuloj. Praktika inĝenierado.*

**Malgrandaj fokusitaj kalkuliloj por la ĉiutaga laboro de elektronika inĝeniero.**

Vi enigas la valorojn, kiujn vi scias, kaj la programo mem provas kalkuli ĉiujn aliajn grandojn: valorojn de rezistiloj, gajnojn, signalajn intervalojn, efikajn kurentojn. Valorojn eblas enigi kun toleremo (`10k 1%`), kaj ĝi estas konsiderata en la kalkuloj. La enkonstruita helpo ne nur klarigas, kiel uzi la programon, sed ankaŭ detale priskribas la ordon de la kalkuloj kun ĉiuj formuloj.

![ECW en uzo: elekto de norma rezistilo, poste projektado de inversa amplifilo per la konataj valoroj](docs/img/demo.gif)

## Ecoj

- **Plenigu tion, kion vi scias.** Neniuj reĝimoj kaj neniu butono *Kalkuli*: enigu ajnan sufiĉan aron da valoroj, kaj la ceteraj estos kalkulitaj.
- **Toleremoj ĉie, kie ili gravas.** Plej malbonkazaj rezultoj per preciza trarigardo de la ekstremaj kombinoj, ne nur optimisma nominalo.
- **Skemoj kiel formularoj.** En la paĝoj de OA kaj nivelŝovilo vi tajpas la valorojn rekte en la cirkviton; la langeto **Plot** montras la transigan linion kaj ĝian toleremzonon.
- **Helpo kiel lernolibro (F1).** Ĉiu kalkulilo havas teorion, la ordon de la kalkulo kun ĉiuj formuloj, laborajn ekzemplojn kaj ligilojn al la dokumentoj kaj normoj, sur kiuj ĝi baziĝas.
- **Inĝeniera enigo.** SI-prefiksoj (`4.7k`, `100n`, `2.2M`), intervaloj (`-5; -1`), malsimetriaj toleremoj (`10k +5% -1%`).
- **Angla, rusa kaj Esperanto**: la interfaco kaj la helpo.
- **Senrete funkcianta programo** por Windows, macOS kaj Linux.

## Kalkuliloj

| Kalkulilo | Kion ĝi faras | Bazo |
|---|---|---|
| **Leĝo de Ohm** | ajnaj du grandoj el U, I, R, P donas ĉiujn aliajn | |
| **Tensiodividilo** | du aŭ tri rezistiloj; ajnaj tri (aŭ kvar) grandoj donas ĉiujn aliajn | |
| **Elekto de rezistiloj** | la plej proksima norma valoro kaj la plej bonaj seriaj kaj paralelaj paroj el E6…E96 aŭ el via propra aro de rezistiloj | IEC 60063 |
| **Tabelo AWG** | rezisto, tensiofalo, potenco kaj varmiĝo de kupra drato, de 40 AWG ĝis 4/0 | ASTM B258 |
| **Formo de kurento** | efika kaj meza valoro de kurento, plenigokvociento por ses kurentformoj | |
| **OA (inversa, neinversa, diferenca)** | gajno, rezistiloj kaj signalaj intervaloj; toleremoj estas konsiderataj | diferenca: TI [SBOA274](https://www.ti.com/lit/SBOA274) |
| **Nivelŝovilo (inversa, neinversa)** | kalkulas la gajnon kaj la nivelŝovon de la enira signalo, sugestas la bezonatan valoron de la referenca tensio | TI [SBOA261](https://www.ti.com/lit/SBOA261), [SBOA262](https://www.ti.com/lit/SBOA262) |

| Diferenca amplifilo | Grafikaĵo kun la toleremzono |
|---|---|
| ![Diferenca amplifilo](docs/img/difference-amplifier.png) | ![Eligo laŭ enigo kun la plej malbonkaza zono](docs/img/plot.png) |

![Paĝo de la enkonstruita helpo: derivaĵo kaj projekta proceduro de la neinversa nivelŝovilo](docs/img/help.png)

## Instalado

Elŝutu la dosieron por via sistemo de la paĝo [Releases](https://github.com/zvladimir/ecw/releases).

Nenio instaliĝas: ĉiu eldono havas Windows-an `.exe` kaj Linuksan `.AppImage`, kiuj funkcias tiel, kiel ili estas (kaj multe pli malgrandan simplan Linuksan binaron `ecw-<versio>-linux-x86_64` por sistemoj, kiuj jam havas WebKitGTK 4.1 kaj GTK 3: `chmod +x` kaj lanĉo), `SHA256SUMS`, kaj `ecw-<versio>-web.html`: la samajn kalkulilojn kiel UNU html-dosieron kun WebAssembly interne, por komputiloj, kie programoj ne povas startiĝi. Malfermu ĝin per duobla alklako en ajna nuna retumilo: nek servilo, nek interreto necesas.

## Kompilado el fontkodo

Vi bezonas [Rust](https://www.rust-lang.org/tools/install) 1.88 aŭ pli novan kaj la sistemajn bibliotekojn de Tauri 2 (en Linukso: WebKitGTK kaj aliaj, vidu la [postulojn de Tauri](https://v2.tauri.app/start/prerequisites/)).

```sh
git clone https://github.com/zvladimir/ecw.git
cd ecw
cargo run -p ecw                    # lanĉi la programon
cargo test -p ecw-core              # unuaj, golden- kaj kontraktaj testoj
cargo build -p ecw --release        # optimumigita binaro en target/release
```

La eldonajn dosierojn kreas la Tauri CLI: `cargo install tauri-cli`, poste `cargo tauri build --no-bundle` (nuda plenumebla dosiero) aŭ `cargo tauri build --bundles appimage`.

La retumila versio: `rustup target add wasm32-unknown-unknown`, `cargo install wasm-bindgen-cli --locked` (la sama versio kiel `wasm-bindgen` en `Cargo.lock`), poste `./scripts/build-web.sh` kreas `target/web/ecw.html` (Python 3 pakas la unuopan dosieron).

## Kontribuado

Raportoj pri eraroj kun la enigitaj valoroj, la atendita rezulto kaj la fonto de la formulo estas tre bonvenaj. Por aldoni kalkulilon, vidu [CONTRIBUTING.md](.github/CONTRIBUTING.md): ĝi paŝo post paŝo priskribas la fizikon, la API-on, la tekstojn, la paĝon kaj la testojn.

## Historio

Versio 0.2 estas reskribo kiel Tauri-programo. La unua versio, *Electrical Calculations Wizard* 0.1a (Rust + iced, leĝo de Ohm kaj tensiodividilo), restas en la historio de ĉi tiu deponejo.

## Licenco kaj subteno

ECW estas libera programaro laŭ la [MIT-licenco](LICENSE). Se ĝi ŝparas al vi tempon, vi povas subteni la evoluigon en [Boosty](https://boosty.to/desialsi).
