# Pri ECW

ECW estas aro de fokusitaj kalkuliloj por ĉiutaga projektado de elektroniko. Ĝi kreskis el la *TI Analog Engineer's Circuit Cookbook*: ĉiu paĝo prenas la kelkajn nombrojn, kiujn vi konas, trovas tiujn, kiujn vi ne konas, kaj montras la rezulton kune kun ĝia **plej malbonkaza disvastiĝo**.

Tri ideoj trairas la tutan programon:

- **Plenigu tion, kion vi scias.** Vi neniam elektas "reĝimon" nek premas *Kalkuli*. Tajpu valorojn, kaj la rezultoj aperas tuj. Kampoj, kiuj sekvas el la aliaj, plenigas sin mem kaj ŝlosiĝas.
- **Toleremoj estas unuarangaj.** Rezistilo ne estas 10 kΩ, ĝi estas 10 kΩ ±1 %. Kie tio havas sencon, vi povas aldoni toleremon al valoro, kaj la respondo revenas kiel nominala valoro kun minimumo kaj maksimumo.
- **La helpo estas lernolibro.** Ĉiu kalkulilo havas ĉapitron, kiu klarigas la elektronikon malantaŭ ĝi: de kie venas la formuloj, kiel projekti per ili paŝo post paŝo, kio misfunkcias en realaj cirkvitoj, kaj laboritaj ekzemploj, kiujn vi povas ripeti en la programo. La ĉapitroj montras la aplikajn notojn kaj normojn, sur kiuj ili baziĝas.

Kiel ĉi tiu helpo estas organizita: *Rapida komenco* klarigas la interfacon, *Fundamentoj* rememorigas la leĝojn uzatajn ĉie, poste sekvas unu ĉapitro por ĉiu kalkulilo (de la simplaj ĝis la cirkvitoj kun operacia amplifilo), kaj la *Referenco* fine kolektas tabelojn, formulojn, normojn kaj dokumentojn.

> **Konsileto.** La helpo malfermiĝas en propra fenestro (butono **Helpo** aŭ klavo **F1**), do vi povas teni ĝin apud la kalkulilo. Teksto en ĉi tiu fenestro povas esti elektita kaj kopiita, kaj substrekitaj ligiloj malfermas la citatajn dokumentojn en via retumilo.

# Rapida komenco

## La fenestro

La fenestro havas tri areojn:

- **Kapo.** La nomo **ECW** kaj kvar butonoj: **Ŝatata** (aldonas la nunan kalkulilon al la listo de plej ŝatataj), **Agordoj**, **Helpo** kaj **Subteni**.
- **Maldekstra menuo.** Serĉkesto kaj la listo de kalkuliloj grupigitaj laŭ kategorio (*Analogaj*, *Bazaj*). Klaku nomon de kategorio por faldi ĝin. Kalkuliloj markitaj kiel ŝatataj ripetiĝas en bloko **★ Plej ŝatataj** tute supre.
- **Paĝo de la kalkulilo.** Ĝia titolo, unulinia priskribo kun la ĉefa formulo, kaj poste la paĝo mem. La simplaj kalkuliloj estas formularoj: **Enirdatumoj** (la kampoj, en kiujn vi tajpas) kaj **Rezultoj**. La kalkuliloj kun operacia amplifilo estas desegnitaj kiel **skemoj kun la kampoj sur ili** kaj havas duan langeton, **Grafikaĵo**.

Ĉio, kion vi tajpas, memoriĝas inter seancoj, do la programo malfermiĝas ĝuste tie, kie vi ĝin lasis. La butono **⌫** en la angulo de paĝo forviŝas ĝian formularon.

## Enmeto de nombroj

Tajpu simplajn nombrojn, laŭvole kun **SI-prefikso** kiel lasta signo. **Ne tajpu unuojn**: la kampo jam scias, ke ĝi atendas voltojn, omojn ktp., kaj ĝia etikedo montras la unuon inter kvadrataj krampoj, ekzemple *R1 [Ω]*.

| Vi tajpas | Signifo                          |
|-----------|----------------------------------|
| `4.7k`    | 4 700 (kilo, $10^3$)             |
| `2.2M`    | 2 200 000 (mega, $10^6$)         |
| `500m`    | 0.5 (milli, $10^{-3}$)           |
| `10u`     | 0.00001 (mikro, $10^{-6}$)       |
| `100n`    | $10^{-7}$ (nano)                 |
| `1e-3`    | 0.001 (scienca skribo)           |
| `-7`      | negativaj nombroj estas permesataj |

> **Atentu pri litergrandeco.** `m` estas *milli* kaj `M` estas *mega*: `5m` estas 0.005, sed `5M` estas kvin milionoj. `G` estas giga, `k` estas kilo, `u` estas mikro, `n` estas nano, `p` estas piko. La prefiksoj estas tiuj de la SI (vidu *Referenco*).

Sur formularaj paĝoj griza linio sub ĉiu kampo montras, kiel la programo komprenis vian enmeton (ekzemple *→ 4.7000 kΩ*) aŭ, kiam la teksto ne validas, kio malĝustas en ĝi. Ruĝa rando markas kampon kun eraro; sur skemo ŝvebu super la kampo por legi la mesaĝon. La butono **×** forviŝas kampon.

## Toleremoj

Kampoj, kiuj priskribas komponantojn, akceptas toleremon post la nombro, apartigitan per spaceto:

| Vi tajpas       | Signifo                                              |
|-----------------|------------------------------------------------------|
| `10k 1%`        | 10 kΩ, ±1 %                                          |
| `10k +5% -1%`   | 10 kΩ, ĝis 5 % supre kaj ĝis 1 % malsupre            |
| `100 -10%`      | 100 Ω, disvastiĝo nur malsupren                      |

Kiuj kampoj akceptas toleremojn, estas dirite en ĉiu ĉapitro. Ĝenerale rezistiloj kaj referencaj tensioj akceptas, same kiel ĉiuj grandoj de la paĝoj Leĝo de Ohm kaj Tensiodividilo. Signalaj amplekso, gajnoj, tempoj kaj similaj *precizaj* enigoj ne akceptas: tia kampo ruĝiĝas, se vi aldonas procenton.

Kiam toleremoj ĉeestas, la rezultoj montras valorojn **Min** kaj **Maks** kalkulitajn por la plej malbona kazo. Kiel tio fariĝas, estas klarigita en *Fundamentoj*.

## Amplekso

La kalkuliloj kun operacia amplifilo laboras kun signalaj **amplekso**: sensilo, kiu donas −1 V ĝis +1 V, ADC, kiu akceptas 0 ĝis 3.3 V. Amplekso estas tajpata en **unu kampon** kiel du nombroj apartigitaj per punktokomo, la pli malgranda unue:

| Vi tajpas   | Signifo                                                    |
|-------------|------------------------------------------------------------|
| `-1; 1`     | de −1 V ĝis +1 V                                           |
| `0.05; 3.3` | de 50 mV ĝis 3.3 V                                         |
| `5`         | OA (inversa, neinversa): la sola valoro 5 V                |
| `5`         | Diferenca amplifilo, nivelŝoviloj: la amplekso 0…5 V       |

> **Noto.** Unu nombro signifas solan valoron sur la du bazaj paĝoj de OA, kaj amplekson ekde 0 sur la aliaj. Skribu ambaŭ finojn, se vi dubas.

## Kalkulitaj kampoj

Sur la plej multaj paĝoj vi donas *kelkajn* grandojn, kaj la programo determinas la reston. Tuj kiam la enigo sufiĉas, la kampoj, kiujn vi ne plenigis, fariĝas **nurlegeblaj**. Sur formularaj paĝoj ili montras la kalkulitan valoron griza; sur skemaj paĝoj ili restas malplenaj kaj streketitaj, kaj iliaj valoroj estas en la rezulta tabelo. Forviŝu unu el viaj kampoj por denove igi la aliajn redakteblaj.

Se la nombroj, kiujn vi enmetis, kontraŭdiras unu la alian aŭ ne priskribas fizike eblan cirkviton, la areo Rezultoj montras ruĝan mesaĝon anstataŭ tabelo, ekzemple *Vout1 devas esti pli malgranda ol Vin*, kune kun tio, kion ŝanĝi.

## Rezultoj kaj grafikaĵoj

Rezultaj tabeloj havas butonon **Kopii tabelon**. Ĝi kopias la tabelon kiel tabulator-apartigitan tekston, kiu algluiĝas rekte en kalkultabelon aŭ tekstan dokumenton, kun la nombro de decimaloj starigita en Agordoj.

La paĝoj de OA havas langeton **Grafikaĵo**: la eliro kontraŭ la enigo. Ŝvebu super la grafikaĵo por legi valorojn; la montrilo algluiĝas al la plej proksima linio. Klaku nomon en la legendo por kaŝi aŭ montri ĝian linion.

## Agordoj, plej ŝatataj kaj serĉo

- **Agordoj** (dentrada butono) enhavas la **lingvon** (English, Русский, Esperanto), la **etoson** (Malhela, Hela aŭ VS Code), la **skalon** (50–150 %) kaj la nombron de **decimalaj ciferoj** (0–9) en la rezultoj. Ŝanĝoj efikas tuj.
- **Ŝatata** aldonas la malfermitan kalkulilon al la bloko ★ Plej ŝatataj de la maldekstra menuo; klaku la stelon denove por forigi ĝin.
- La **serĉkesto** filtras la menuon laŭ titolo.

## Klavaro

| Klavo     | Ago                                          |
|-----------|----------------------------------------------|
| `Tab`     | iri al la sekva kampo                        |
| `Ctrl+C`  | kopii la elektitan tekston de kampo          |
| `F1`      | malfermi ĉi tiun helpon                      |
| `Esc`     | fermi la dialogon Agordoj aŭ ĉi tiun fenestron |

# Fundamentoj

Ĉi tiu ĉapitro rememorigas la manplenon da leĝoj, sur kiuj la kalkuliloj baziĝas. Se vi bone konas ilin, saltu al la ĉapitro, kiun vi bezonas.

## Tensio, kurento, rezisto

**Tensio** $U$ (voltoj, V) estas la energio por unuo de ŝargo necesa por movi ŝargon inter du punktoj. **Kurento** $I$ (amperoj, A) estas la rapido de fluo de ŝargo. **Rezisto** $R$ (omoj, Ω) diras, kiom da tensio necesas por puŝi donitan kurenton tra komponanto. **La leĝo de Ohm** asertas, ke por rezistilo ili estas proporciaj:

$$U = I \cdot R$$

La inverso de rezisto estas **konduktanco** $G = 1/R$, mezurata en simensoj (S). Ĝi ofte estas pli oportuna por paralelaj cirkvitoj.

## Leĝoj de Kirchhoff

Du konservaj leĝoj permesas analizi ajnan cirkviton el rezistiloj kaj fontoj:

- **Kurenta leĝo (KCL).** La kurentoj, fluantaj en nodon, sumiĝas al nulo: ŝargo ne amasiĝas en drato. Por nodo kun kurentoj $I_1 \dots I_n$ (en la nodon pozitivaj): $\sum I_k = 0$.
- **Tensia leĝo (KVL).** La tensioj ĉirkaŭ ajna fermita buklo sumiĝas al nulo: la potencialo en punkto ne dependas de la vojo, laŭ kiu oni atingas ĝin. $\sum U_k = 0$.

Ĉiu formulo en la ĉapitroj pri OA estas KCL skribita ĉe la enira nodo de la amplifilo.

## Potenco

Elektra **potenco** (vatoj, W) estas tensio oble kurento, kaj kun la leĝo de Ohm ĝi havas tri ekvivalentajn formojn:

$$P = U \cdot I = I^2 \cdot R = \frac{U^2}{R}$$

En rezistilo *la tuta* ĉi tiu potenco fariĝas varmo. Rezistilo taksita je 0.25 W, kiu disipas 0.5 W, trovarmiĝos, do potenco estas kontrolata same zorge kiel rezisto. Potenco kreskas kiel la *kvadrato* de la kurento: duobligo de la kurento kvarobligas la varmon. Ofta projekta regulo estas teni la disipon sub duono de la nominala potenco.

## Seria kaj paralela konekto

![Du rezistiloj serie kaj paralele](fig:series_parallel)

En **seria** konekto la sama kurento fluas tra ambaŭ rezistiloj kaj la tensioj sumiĝas, do la rezistoj sumiĝas:

$$R_{seria} = R_1 + R_2$$

En **paralela** konekto ambaŭ rezistiloj havas la saman tension kaj la kurentoj sumiĝas, do la *konduktancoj* sumiĝas:

$$\frac{1}{R_{paralela}} = \frac{1}{R_1} + \frac{1}{R_2} \quad\Rightarrow\quad R_{paralela} = \frac{R_1 \cdot R_2}{R_1 + R_2}$$

Seria kombino ĉiam estas *pli granda* ol la plej granda rezistilo, paralela kombino ĉiam estas *pli malgranda* ol la plej malgranda. Se unu rezistilo estas multe pli granda ol la alia, la rezulto en paralela cirkvito estas dominata de la pli malgranda, kaj en seria cirkvito de la pli granda.

## Superpozicio

En cirkvito el rezistiloj (ajna *lineara* cirkvito) kun pluraj fontoj la respondo estas la **sumo de la respondoj al ĉiu fonto aparte**, kun la ceteraj tensiaj fontoj anstataŭigitaj per fuŝkontaktoj. Tiel oni analizas la diferencan amplifilon kaj la nivelŝovilojn: la eliro estas "la parto pro la signalo" plus "la parto pro la referenco".

## Tolero kaj plej malbona kazo

Realaj komponantoj neniam estas precizaj. Rezistilo "10 kΩ ±1 %" povas mezuriĝi ie ajn inter 9.9 kΩ kaj 10.1 kΩ. Cirkvito farita el tiaj partoj havas eliron, kiu ankaŭ varias, kaj bona projekto devas funkcii por *ĉiu* kombino de la permesataj valoroj.

La kalkuliloj trovas la **plej malbonan kazon**: la plej malgrandan kaj la plej grandan valoron, kiun rezulto povas preni. Du metodoj estas uzataj.

- **Intervala aritmetiko.** Ĉiu valoro portiĝas kiel tri nombroj, *nominala*, *minimuma* kaj *maksimuma*, kaj ĉiu operacio estas farata sur la limoj tiel, ke la rezulto enhavas ĉiujn eblajn rezultojn. La sumo de $a \in [a_{min}, a_{max}]$ kaj $b \in [b_{min}, b_{max}]$ kuŝas en $[a_{min} + b_{min},\; a_{max} + b_{max}]$; produto aŭ kvociento kuŝas inter la plej malgranda kaj la plej granda el la kvar kombinoj de la limoj; divido per intervalo, kiu enhavas nulon, estas rifuzata. La paĝo Leĝo de Ohm laboras tiel.
- **Angula analizo.** Intervala aritmetiko estas tro pesimisma, kiam la sama komponanto aperas dufoje en formulo: en $V_{in} R_2 / (R_1 + R_2)$ alta $R_2$ altigas la numeratoron kaj la denominatoron kune, do la du efikoj parte nuliĝas, kion intervala aritmetiko ne povas scii. Kiam rezulto ŝanĝiĝas monotone kun ĉiu enigo, ĝiaj ekstremoj kuŝas ĉe la **anguloj**, la kombinoj de la minimumo kaj maksimumo de ĉiu enigo. Por $n$ toleremaj enigoj la programo kalkulas la cirkviton ĉe ĉiuj $2^n$ anguloj kaj prenas la plej malaltan kaj la plej altan rezulton. La paĝoj Tensiodividilo kaj ĉiuj paĝoj de OA laboras tiel, do ilia disvastiĝo estas preciza, ne nur sekura.

La plej malbona kazo estas la sekura takso, kaj pesimisma: realaj aroj de komponantoj statistike estas multe pli bonaj ol la limoj sugestas, kaj estas neprobable, ke ĉiuj partoj samtempe troviĝas ĉe la malfavora fino. Se vi bezonas probablecan takson, uzu la plej malbonan kazon kiel supran limon kaj precizigu ĝin per takso de radiko el sumo de kvadratoj aŭ per Monte-Karlo-simulado.

En la tabeloj la disvastiĝo montriĝas kiel **Min**, **Maks** kaj **Δ, %**, la deviiĝo de la limoj de la nominala valoro (unu nombro kun ±, kiam ili estas simetriaj). La paĝoj Leĝo de Ohm kaj Tensiodividilo montras ĝin pli detale:

| Vico       | Signifo                                                     |
|------------|-------------------------------------------------------------|
| Nominala   | la valoro por ĝuste nominalaj enigoj                        |
| Min / Maks | la plej malgranda kaj la plej granda ebla valoro            |
| Δ + %      | kiom super la nominala kuŝas la maksimumo, en procentoj     |
| Δ + val    | la sama, kiel absoluta kvanto                               |
| Δ - %      | kiom sub la nominala kuŝas la minimumo, en procentoj        |
| Δ - val    | la sama, kiel absoluta kvanto                               |

## Preferataj valoroj: la E-vicoj

Rezistiloj ne estas produktataj en ĉiu valoro. Fabrikantoj ofertas valorojn el la **vicoj de preferataj nombroj** de la normo **IEC 60063**, nomitaj **E6, E12, E24, E48, E96, E192** laŭ la nombro de valoroj en dekado. La valoroj estas geometrie distancigitaj:

$$R_k \approx 10^{k/N}, \qquad k = 0, 1, \dots, N-1$$

poste rondigitaj al du (E6–E24) aŭ tri (E48–E192) signifaj ciferoj kaj ripetitaj en ĉiu dekado. Por E12 la proporcio inter najbaroj estas $10^{1/12} \approx 1.21$, paŝo de ĉirkaŭ 21 %. La rondigo ne ĉiam estas la evidenta: E24 enhavas 2.7, 3.0, 3.3 … 4.7, 8.2, kiuj ne sekvas $10^{k/24}$ laŭ du ciferoj, kaj E192 listigas 9.20, kvankam $10^{185/192}$ rondiĝas al 9.19. Uzu la norman tabelon, ne la formulon, kiam la preciza valoro gravas.

![La vicoj E6, E12 kaj E24 sur logaritma skalo](fig:eseries)

La distanco estas elektita tiel, ke najbaraj valoroj *ĝuste interkovriĝas*, kiam oni konsideras la toleremon: ajna rezisto kuŝas ene de la tolero de iu preferata valoro.

| Vico   | Valoroj en dekado | Tipa tolero       |
|--------|-------------------|-------------------|
| E6     | 6                 | 20 %              |
| E12    | 12                | 10 %              |
| E24    | 24                | 5 % (kaj 1 %)     |
| E48    | 48                | 2 %               |
| E96    | 96                | 1 %               |
| E192   | 192               | 0.5 % kaj pli bone |

Kiam kalkulo donas "malbelan" valoron kiel 26.36 kΩ, elektu la plej proksiman preferatan valoron aŭ kombinu du rezistilojn por veni pli proksimen. La **Elekto de rezistiloj** faras ĝuste tion. Sur la paĝoj de OA vi povas tajpi normvaloron en kalkulitan kampon kaj tuj vidi, kiel ĝi movas la rezulton.

# Leĝo de Ohm

**Kategorio:** Bazaj. La plej simpla kalkulilo en la programo, kaj tiu, kun kiu komenci.

## Kion ĝi faras

Vi konas du el kvar grandoj, tension $U$, kurenton $I$, reziston $R$ kaj potencon $P$, kaj la paĝo kalkulas la aliajn du same kiel la konduktancon $G$. Ĝi realigas la ses kombinojn de du konataj grandoj:

| Vi scias | Uzataj formuloj                                              |
|----------|--------------------------------------------------------------|
| $U$, $I$ | $R = U / I$,  $P = U \cdot I$                                 |
| $U$, $R$ | $I = U / R$,  $P = U \cdot I$                                 |
| $U$, $P$ | $I = P / U$,  $R = U / I$                                     |
| $I$, $R$ | $U = I \cdot R$,  $P = U \cdot I$                             |
| $I$, $P$ | $U = P / I$,  $R = U / I$                                     |
| $R$, $P$ | $U = \sqrt{P \cdot R}$,  $I = \sqrt{P / R}$                   |

## Kiel uzi ĝin

1. Enmetu **ĝuste du** grandojn. La aliaj du kampoj griziĝas.
2. Legu la rezultan tabelon: ĝi enhavas ĉiujn kvin grandojn $U$, $I$, $R$, $G$ kaj $P$ kun iliaj nominalaj valoroj kaj disvastiĝo.

Ĉiuj kvar enigoj estas pozitivaj nombroj kaj akceptas toleremojn, ekzemple `4.7k 5%`. Se vi plenigas tri kampojn, la paĝo avertas *Donite 3 — necesas ĝuste 2*; kun unu kampo ĝi petas ankoraŭ unu.

## Laborita ekzemplo: kurent-limiga rezistilo

LED bezonas 10 mA kaj falas 2 V. Ĝi estas nutrata de 5 V. La rezistilo devas fali la ceterajn 3 V portante 10 mA, do enmetu $U = 3$ kaj $I = 10\text{m}$. La paĝo redonas $R = 300\ \Omega$ kaj $P = 30\ \text{mW}$. La plej proksima valoro de E24 estas 300 Ω mem; peco de 0.1 W plene sufiĉas (nominalaj potencoj normale estas elektataj almenaŭ 2× super la disipo).

Nun provu $R = 330$ kun `5%` kaj $U = 3$: la kurento rezultas inter 8.66 mA kaj 9.57 mA, kaj la tabelo montras la plej malbonkazan disvastiĝon, do vi povas kontroli, ke la LED estas nek tro malhela nek troŝargita.

> **Memoru.** La proporcio de la plej malbonkaza potenco al la nominala potenco gravas pli ol la nominala cifero: ĉiam legu la kolumnon **Maks** de la vico $P$, kiam vi elektas parton.

# Tensiodividilo

**Kategorio:** Bazaj. Paro da rezistiloj, kiu produktas pli malaltan tension el pli alta: la plej ofta cirkvito en elektroniko.

## La cirkvito

![Durezistila tensiodividilo](fig:divider2)

La sama kurento $I$ fluas tra ambaŭ rezistiloj, do

$$I = \frac{V_{in}}{R_1 + R_2}, \qquad V_{out} = I \cdot R_2 = V_{in} \cdot \frac{R_2}{R_1 + R_2}$$

La proporcio $R_2 / (R_1 + R_2)$ ĉiam estas inter 0 kaj 1, do dividilo povas nur *redukti* tension. La eliro estas frakcio de la enigo, kaj la frakcio dependas nur de la *proporcio* de la rezistiloj, ne de iliaj absolutaj valoroj. Tial vi havas liberan elekton de skalo, kiun vi uzas por kompromisi inter precizeco kaj konsumo de potenco (vidu sube).

## Reĝimo kun du rezistiloj

Elektu **2 rezistiloj** en la listo *Konfiguro* kaj donu **ajnajn tri** el la kvar grandoj $V_{in}$, $V_{out}$, $R_1$, $R_2$. La kvara estas kalkulata:

| Vi scias                | Kalkulita                                             |
|-------------------------|-------------------------------------------------------|
| $V_{in}$, $R_1$, $R_2$  | $V_{out} = V_{in} R_2 / (R_1 + R_2)$                  |
| $V_{in}$, $V_{out}$, $R_1$ | $R_2 = R_1 V_{out} / (V_{in} - V_{out})$           |
| $V_{in}$, $V_{out}$, $R_2$ | $R_1 = R_2 (V_{in} - V_{out}) / V_{out}$           |
| $V_{out}$, $R_1$, $R_2$ | $V_{in} = V_{out} (R_1 + R_2) / R_2$                  |

Se $V_{out} \ge V_{in}$, la paĝo raportas *Vout devas esti pli malgranda ol Vin*.

## Reĝimo kun tri rezistiloj

![Trirezistila tensiodividilo kun du kranoj](fig:divider3)

Kun tri rezistiloj la dividilo havas du kranojn:

$$V_{out1} = V_{in} \cdot \frac{R_2 + R_3}{R_1 + R_2 + R_3}, \qquad V_{out2} = V_{in} \cdot \frac{R_3}{R_1 + R_2 + R_3}$$

Elektu **3 rezistiloj** kaj donu **ajnajn kvar** el la ses grandoj. La paĝo solvas la ceterajn du. Tipaj uzoj estas referencaj ŝtuparoj por komparatoroj (supraj kaj malsupraj sojloj), kontroliloj de bateria tensio kaj biasaj retoj. La orda regulo $V_{in} > V_{out1} > V_{out2} > 0$ ĉiam validas kaj estas kontrolata.

## Elekto de la valoroj de la rezistiloj

Ajna proporcio povas esti konstruita el senfine multaj paroj de rezistiloj. La dividilo ĉiam tiras $V_{in}^2 / (R_1 + R_2)$ el la fonto: por 12 V-a relo tra $2\ \text{k}\Omega$ tio estas 72 mW, malŝparataj senĉese; tra $200\ \text{k}\Omega$ ĝi estas nur 0.72 mW. Bateriaj cirkvitoj bezonas grandajn valorojn; por precizaj cirkvitoj 1–100 kΩ estas tipa elekto.

## Tolero de la proporcio

Toleremoj de rezistiloj ŝovas la eliron. Se $R_1$ kaj $R_2$ ambaŭ havas toleron $t$, la plej malbona kazo okazas, kiam unu estas ĉe la supro kaj la alia ĉe la fundo de sia amplekso, kaj la relativa eraro de la proporcio $k = R_2 / (R_1 + R_2)$ estas maksimume

$$\frac{\Delta k}{k} \approx 2\,t\,(1 - k)$$

Por $k = 0.5$ kaj 1 % rezistiloj la eliro povas erari je 1 %; por dividilo kun $k = 0.1$ ĝi povas erari je 1.8 %. La paĝo aplikas toleremojn aŭtomate: aldonu `1%` al la kampoj de rezistiloj kaj komparu la kolumnojn **Min** kaj **Maks** de $V_{out}$.

## Laborita ekzemplo: 12 V al 3.3 V

Vi bezonas 3.3 V el 12 V-a relo kaj decidas pri $R_2 = 10\ \text{k}\Omega$.

1. **Ideala valoro.** Enmetu $V_{in} = 12$, $V_{out} = 3.3$ kaj $R_2 = 10\text{k}$. La paĝo kalkulas $R_1 = R_2 (V_{in} - V_{out}) / V_{out} = 10\ \text{k}\Omega \cdot 8.7 / 3.3 = 26.36\ \text{k}\Omega$, kiu ne estas norma valoro.
2. **Norma valoro.** En la **Elekto de rezistiloj** enmetu `26.3636k` kaj elektu E96: la plej proksima valoro estas 26.1 kΩ. Tiam $V_{out} = 12 \cdot 10 / 36.1 = 3.324$ V, 0.7 % super la celo.
3. **Tolero.** Reen en la dividilo enmetu $R_1 = 26.1\text{k}\ 1\%$ kaj $R_2 = 10\text{k}\ 1\%$: la eliro kuŝas inter 3.276 V kaj 3.372 V (−1.44 %, +1.45 %), proksime al la takso $2t(1-k) = 2 \cdot 1\,\% \cdot 0.72 = 1.44\,\%$.
4. **Kurento kaj potenco.** $I = 12 / 36.1\ \text{k}\Omega = 0.33$ mA; la dividilo disipas $12^2 / 36.1\ \text{k}\Omega = 4$ mW, nenio por peco de 0.1 W.

# Elekto de rezistiloj

**Kategorio:** Bazaj. Trovas la plej bonan realigon de arbitra rezisto per normaj valoroj.

## Kion ĝi faras

Vi enmetas **celan reziston** kaj elektas **E-vicon** (vidu *Fundamentoj*). Aldone vi povas tajpi vian propran **aron**: la liston de rezistiloj, kiujn vi vere havas en la tirkesto, apartigitajn per komoj, punktokomoj aŭ spacetoj, kun SI-prefiksoj, ekzemple `10k, 2k, 100`. Malvalidaj aŭ duoblaj enskriboj estas ignorataj.

Por kaj la elektita vico kaj la propra aro la paĝo montras tri realigojn:

| Vico           | Signifo                                                              |
|----------------|----------------------------------------------------------------------|
| Unu (1×)       | la plej proksima unuopa rezistilo                                    |
| Seria (2×)     | la paro $R_a + R_b$, kies sumo estas plej proksima al la celo        |
| Para (2×)      | la paro $R_a \parallel R_b$, kies kombino estas plej proksima        |

Ĉiu ĉelo montras la valorojn kaj la rezultan reziston, kaj sub ĝi la **deviiĝon** de la celo en procentoj. Verda **✓** signifas, ke la eraro estas ene de la tipa tolero de la vico (20 % por E6, 10 % por E12, 5 % por E24, 2 % por E48, 1 % por E96); flava **⚠** signifas, ke ne. Por la propra aro oni uzas la saman sojlon de la elektita vico kiel mezurilon.

## Kiel kombinoj helpas

Kombino alproksimiĝas al la celo pli ol ajna unuopa norma valoro. Du ideoj indas scii:

- **Seria alĝustigo.** Por atingi 10.12 kΩ elektu grandan rezistilon ĝuste sub la celo kaj aldonu malgrandan: 10 kΩ + 120 Ω donas precize 10.12 kΩ. La malgranda rezistilo alĝustigas la rezulton kaj kontribuas malmulte al la tuta tolero.
- **Paralela alĝustigo.** Meti multe pli grandan rezistilon paralele *malaltigas* la valoron iomete: $10\ \text{k}\Omega \parallel 1\ \text{M}\Omega = 9.9\ \text{k}\Omega$. Tio utilas, kiam disponeblas nur valoroj iom super la celo.

Por rezistiloj de la sama tipo kaj tolero la relativa tolero de seria aŭ paralela kombino ne estas pli malbona ol tiu de ĝiaj partoj, kaj la kontribuo de malgranda alĝustiga rezistilo estas proporcie pli malgranda. Kombinoj kostas lokon sur la cirkvitplato kaj ankoraŭ unu lutadon, do uzu ilin nur kiam precizeco postulas, ekzemple por la reagaj dividiloj de reguliloj aŭ por precizaj gajnoj.

## Laborita ekzemplo

Implicitaj valoroj: celo 10.12 kΩ, E24. La rezultoj: la plej proksima unuopa valoro de E24 estas 10 kΩ (−1.19 %, ✓), la plej bona seria paro estas 10 kΩ + 120 Ω (+0.00 %), kaj la plej bona paralela paro 11 kΩ ∥ 130 kΩ donas 10.14 kΩ (+0.22 %). Kun la propra aro `10k, 2k, 100` unuopa 10 kΩ denove havas eraron −1.19 %, kaj la plej bona paro estas 10 kΩ + 100 Ω = 10.1 kΩ (−0.20 %).

# Tabelo AWG

**Kategorio:** Bazaj. Referenco pri kupra drato de ĉiu ofta amerika dratmezuro (American Wire Gauge), de 40 AWG ĝis 0000 (4/0), kun la tensiofalo, disipata potenco kaj temperaturaltiĝo por *via* kurento kaj longo.

## Dratmezuro

**AWG** (American Wire Gauge) numeras dratojn laŭ tio, kiom da fojoj ili estis tiritaj tra sinsekvaj tirfiloj: ju *pli granda* la numero, des *pli maldika* la drato. La diametro sekvas geometrian leĝon inter 36 AWG (0.127 mm) kaj 0000 AWG (11.68 mm):

$$d_n = 0.127\ \text{mm} \cdot 92^{\,(36 - n)/39}$$

Utilaj ĉirkaŭkalkuloj, kiuj sekvas el tio:

- ĉiun **6 mezurojn** la diametro duobliĝas (aŭ duoniĝas);
- ĉiun **3 mezurojn** la sekco, kaj do la konduktanco, duobliĝas;
- ĉiun **10 mezurojn** la sekco ŝanĝiĝas je faktoro 10.

La sekco estas $A = \pi d^2 / 4$. La tabelo uzas la nominalajn diametrojn de la norma **ASTM B258**. Ekster Nordameriko konduktoroj estas specifataj per sia sekco en mm² (**IEC 60228**: 0.5, 0.75, 1, 1.5, 2.5 mm² ktp.): 0.5 mm² estas proksima al 20 AWG, 1.5 mm² al 16 AWG, 2.5 mm² al 14 AWG.

## Rezisto, falo kaj hejtiĝo

La rezisto de unuforma konduktoro kun longo $L$ kaj sekco $A$ farita el materialo kun rezistivo $\rho$ estas

$$R = \rho \frac{L}{A}$$

La kalkulilo uzas $\rho = 1.72 \cdot 10^{-8}\ \Omega\text{m}$, la rezistivon de kalcinita kupro je 20 °C (la internacia kalcinita kupra normo, **IEC 60028**, difinas $1/58\ \Omega\,\text{mm}^2/\text{m} = 1.7241 \cdot 10^{-8}\ \Omega\text{m}$). Se la konduktoro estas fasko de $n$ fadenoj paralele, la sekco estas multiplikata per $n$ kaj la rezisto malaltiĝas je la sama faktoro. Kurento $I$ tiam kaŭzas

$$V_{falo} = I \cdot R, \qquad P = I^2 \cdot R, \qquad \Delta T = P \cdot R_{th}$$

kie $R_{th}$ estas la **termika rezisto** en K/W: kiom da kelvinoj la drato varmiĝas por ĉiu vato disipata en ĝi, kio dependas de izolo, muntado kaj aerfluo. La temperaturaltiĝo estas relativa al la ĉirkaŭaĵo.

## Kiel uzi ĝin

1. Enmetu la kurenton $I$ (A), la longon $L$ (m) kaj la termikan reziston $R_{th}$ (K/W) de la drato. La implicitaj valoroj estas 1 A, 1 m kaj 50 K/W. Ĉiuj tri estas simplaj pozitivaj nombroj sen tolero.
2. La tabelo listigas ĉiun mezuron kun ĝia diametro $D$, sekco $A$, rezisto $R$, tensiofalo $V_{falo}$, potenco $P$ kaj temperaturaltiĝo $\Delta T$.
3. Se vi uzas fadenan faskon, ŝanĝu la valoron **n** en la lasta kolumno de la vico: la rezisto, falo, potenco kaj hejtiĝo de tiu vico estas rekalkulataj por $n$ paralelaj dratoj.

Rigardu laŭ la kolumno de la grando, kiun vi zorgas, ekzemple la falo ĉe la fino de longa nutra linio, kaj elektu la plej maldikan dratmezuron, kiu ankoraŭ plenumas la limon.

## Laborita ekzemplo

2 m longa nutra kablo portas 1 A al sensilo. Por 24 AWG ($d = 0.511$ mm, $A = 0.205\ \text{mm}^2$) la rezisto de 1 m estas $0.084\ \Omega$. Enmetu $L = 4$ (la kurento fluas *tien kaj reen*, do la buklo estas 4 m): $R = 0.336\ \Omega$, $V_{falo} = 0.336$ V, $P = 0.336$ W. Kun $R_{th} = 50$ K/W la drato varmiĝas je ĉirkaŭ 17 K, kio estas akceptebla. La sama buklo el 30 AWG falus 1.35 V kaj varmiĝus je 68 K, kio ne estas akceptebla.


# Formo de kurento

**Kategorio:** Bazaj. Kalkulas la **efektivan** (RMS) kaj la **mezan** valoron de perioda kurenta impulso, kaj ĝian plenigfaktoron, por ses oftaj ondoformoj.

## Kial RMS kaj meza

La kurento en povelektroniko malofte estas konstanta: ĝi estas vico da impulsoj. Malsamaj efikoj dependas de malsamaj ecoj de la ondoformo:

- **Hejtiĝo** en rezistaj partoj (dratoj, kanalo de MOSFET, volvaĵo de induktilo, ESR de kondensilo) estas $P = I_{rms}^2 R$. La **RMS** (radiko de meza kvadrato) kurento estas *difinita* kiel la kontinua kurento, kiu disipus la saman potencon:

$$I_{rms} = \sqrt{\frac{1}{T} \int_0^T i^2(t)\, dt}$$

- **La meza valoro** $I_{avg}$ determinas, kion montras kontinua mezurilo, la ŝargon liveratan al baterio, kaj la perdon en aparato kun fiksa tensiofalo, kiel diodo: $P = V_f \cdot I_{avg}$.
- **La pika valoro** determinas, ĉu induktilo saturiĝas kaj ĉu limo de fuzeo aŭ duonkonduktilo estas superata.

La **plenigfaktoro** $D = t_{on} / T$ estas la frakcio de la periodo, en kiu fluas kurento. Du pliaj indikoj de kvalito estas la **pika faktoro** $I_p / I_{rms}$ kaj la **formfaktoro** $I_{rms} / I_{avg}$. Por konstanta kurento ambaŭ egalas 1; ju pli akraj la impulsoj, des pli grandaj ili fariĝas.

## La ses ondoformoj

Elektu la ondoformon en la listo *Formo*. La antaŭvido dekstre redesegniĝas dum vi tajpas; por malplenaj kampoj ĝi montras specimenan formon, do vi ĉiam vidas, kio estas kalkulata. En la grafikaĵo la blua linio estas $I_{rms}$ kaj la oranĝa linio estas $I_{avg}$; dimensiaj sagoj montras la tempojn kaj la pikan kurenton.

### Duonsinusa

![Duonsinusa impulso](fig:wave_half_sine)

Enigoj: piko $I_p$, impulsa larĝo $t_{on}$, periodo $T$. Tio estas la formo de rektigita sinuso kaj de la kurento en resonaj konvertiloj.

$$D = \frac{t_{on}}{T}, \qquad I_{rms} = I_p \sqrt{\frac{D}{2}}, \qquad I_{avg} = \frac{2}{\pi} I_p D$$

### Rektangula

![Rektangula impulso](fig:wave_rect)

Enigoj: piko $I_p$, $t_{on}$, $T$. La klasika PWM-kurento.

$$I_{rms} = I_p \sqrt{D}, \qquad I_{avg} = I_p D$$

La kvadrata radiko estas la ŝlosila observo: impulso kun 25 % plenigfaktoro havas RMS **50 %** de sia pikvaloro sed mezan nur 25 %.

### Trapeza

![Trapeza impulso](fig:wave_trapezoid)

Enigoj: kurento ĉe la komenco $I_{p1}$ kaj ĉe la fino $I_{p2}$ de la impulso, $t_{on}$ kaj $T$. La kurento ŝanĝiĝas linie dum la impulso, kiel en la kurento de la induktilo de malaltiga konvertilo en kontinua reĝimo.

$$I_{rms} = \sqrt{D \cdot \frac{I_{p1}^2 + I_{p1} I_{p2} + I_{p2}^2}{3}}, \qquad I_{avg} = D \cdot \frac{I_{p1} + I_{p2}}{2}$$

Por $I_{p1} = I_{p2}$ tio reduktiĝas al la rektangula impulso, kaj por $I_{p1} = 0$ al segdento.

### Triangula

![Triangula impulso](fig:wave_triangular)

Enigoj: piko $I_{max}$ kaj la tempoj $t_1$ (kresko), $t_2$ (malkresko), $t_3$ (paŭzo) kaj $T$. **Enmetu ajnajn tri el $t_1$, $t_2$, $t_3$ kaj $T$:** la kvara kampo griziĝas kaj montras la valoron, kiu sekvas el $T = t_1 + t_2 + t_3$.

$$D = \frac{t_1 + t_2}{T}, \qquad I_{rms} = I_{max}\sqrt{\frac{D}{3}}, \qquad I_{avg} = \frac{I_{max}}{2}\, D$$

### Triangula kun ŝovo

![Triangula impulso kun ŝovo](fig:wave_offset)

La kurento kreskas de planko $I_{min}$ ĝis $I_{max}$ en $t_1$ kaj malkreskas reen en $t_2$. Tio estas la formo de ondeto-kurento de induktilo en konvertilo, kiu ne iras al nulo. Tempoj estas enmetataj kiel por la triangula ondoformo; $I_{min}$ devas esti pozitiva kaj sub $I_{max}$.

$$I_{rms} = \sqrt{D \cdot \frac{I_{min}^2 + I_{min} I_{max} + I_{max}^2}{3}}, \qquad I_{avg} = D \cdot \frac{I_{min} + I_{max}}{2}$$

### Dupolusa triangula

![Dupolusa triangula impulso](fig:wave_bipolar)

Negativa triangulo kun piko $I_{min}$ (*negativa* nombro) daŭranta $t_1$, sekvata de pozitiva triangulo kun piko $I_{max}$ daŭranta $t_2$, kiel en la volvaĵa kurento de transformilo en dutakta ŝtupo. La pozitivaj kaj negativaj loboj parte nuliĝas en la meza valoro, sed ambaŭ hejtas la volvaĵon:

$$I_{avg} = \frac{I_{min}\, t_1 + I_{max}\, t_2}{2\,T}, \qquad I_{rms} = \sqrt{\frac{I_{min}^2\, t_1 + I_{max}^2\, t_2}{3\,T}}$$

## Rezultoj

La tabelo Rezultoj donas $I_{rms}$, $I_{avg}$ kaj la plenigfaktoron. La enigoj estas precizaj nombroj (toleremoj ne estas akceptataj). La paĝo kontrolas, ke la geometrio eblas: impulso ne povas esti pli longa ol la periodo (*la daŭro de la impulso ne devas superi la periodon*), tempoj devas esti pozitivaj, kaj la pika kurento de la impulsaj ondoformoj ne povas esti negativa.

## Laborita ekzemplo

MOSFET ŝaltas 2 A impulsojn de 1 ms ĉiujn 4 ms. Elektu **Rektangula** kaj enmetu $I_p = 2$, $t_{on} = 1\text{m}$, $T = 4\text{m}$. La paĝo donas $D = 0.25$, $I_{rms} = 1$ A, $I_{avg} = 0.5$ A. Kun $R_{DS(on)} = 50\ \text{m}\Omega$ la kondukta perdo estas $P = I_{rms}^2 R_{DS(on)} = 50$ mW, *ne* la $0.2$ W, kiun vi ricevus el la pika kurento, kaj ne la 12.5 mW el la meza kurento. La sama impulso tra diodo kun $V_f = 0.7$ V disipas $0.7 \cdot 0.5 = 0.35$ W.

> **Regulo.** Uzu $I_{rms}$ por ĉio rezista, $I_{avg}$ por ĉio kun fiksa tensiofalo (diodoj, saturitaj transistoroj, baterioj) kaj $I_p$ por saturiĝo kaj absolutaj limoj.


# Operaciaj amplifiloj

La kategorio *Analogaj* enhavas kvin kalkulilojn de OA: la **inversan** kaj **neinversan** amplifilojn, la **diferencan amplifilon** kaj du **nivelŝovilojn**. Ĉi tiu ĉapitro klarigas la operacian amplifilon mem kaj la du bazajn amplifilojn; la sekvaj ĉapitroj baziĝas sur ĝi.

## La ideala OA

Operacia amplifilo estas diferenca amplifilo kun enorma malferma-bukla gajno $A$ (tipe $10^5$–$10^7$) inter ĝia neinversa enigo (+) kaj ĝia inversa enigo (−):

$$V_{out} = A\,(V_+ - V_-)$$

Amplifilo kun tia gajno utilas nur ene de **negativa reaga buklo**: parto de la eliro estas resendata al la inversa enigo, tiel ke la buklo devigas $V_+ - V_- = V_{out}/A$ esti praktike nula. Du supozoj tiam priskribas la OA:

1. **La enigoj ne tiras kurenton.**
2. **La tensio inter la enigoj estas nula**: la *virtuala fuŝkontakto*. La reago alĝustigas la eliron, ĝis tio veras.

Ĉiuj formuloj de la ĉapitroj pri OA sekvas el ĉi tiuj du reguloj, la leĝo de Ohm kaj la kurenta leĝo de Kirchhoff ĉe la inversa enigo. La TI-projektoj sube tenas la rezistilojn en la amplekso 1–100 kΩ.

## Inversa amplifilo

![Inversa amplifilo](fig:opamp_inv)

La neinversa enigo estas surterigita, do laŭ la virtuala fuŝkontakto la inversa enigo ankaŭ estas je 0 V, *virtuala tero*. KCL ĉe tiu nodo: la enira kurento $V_{in}/R_1$ ne povas eniri la OA, do ĝi tuta fluas tra la reaga rezistilo $R_2$, kaj $V_{out} = -R_2 \cdot V_{in}/R_1$:

$$G = \frac{V_{out}}{V_{in}} = -\frac{R_2}{R_1}$$

La gajno estas negativa: la eliro estas la enigo renversita. Ĝia grando povas esti pli malgranda aŭ pli granda ol 1. La enira rezisto de la ŝtupo estas nur $R_1$, do la fonto devas povi alimenti tiun ŝarĝon.

## Neinversa amplifilo

![Neinversa amplifilo](fig:opamp_noninv)

Ĉi tie la signalo iras rekte al la + enigo. La reaga dividilo $R_1$ (de la eliro) kaj $R_2$ (al tero) resendas la frakcion $R_2 / (R_1 + R_2)$ de la eliro al la − enigo. La virtuala fuŝkontakto egaligas ĉi tiun frakcion kun $V_{in}$:

$$V_{in} = V_{out} \frac{R_2}{R_1 + R_2} \quad\Rightarrow\quad G = 1 + \frac{R_1}{R_2}$$

La gajno estas pozitiva kaj **almenaŭ 1**. La enira rezisto estas ekstreme alta, ĉar la signalo renkontas nur la enigon de la OA, kio igas ĉi tiun ŝtupon ideala por bufrado de malfortaj fontoj. $R_1 = 0$ kaj $R_2 = \infty$ donas la *tensiosekvilon* kun $G = 1$; la paĝo postulas $G > 1$ kaj ne kovras ĝin.

> **Nomoj de rezistiloj.** Sur ĉi tiuj du paĝoj $R_1$ kaj $R_2$ estas nomitaj kiel en la plej multaj lernolibroj: $R_1$ estas la enira rezistilo de la inversa ŝtupo kaj la reaga rezistilo de la neinversa ŝtupo. La sekvaj ĉapitroj uzas la nomojn de la TI-dokumentoj, sur kiuj ili baziĝas, kie $R_1$ ĉiam estas la reaga rezistilo.

## La paĝoj de OA

![Eliro de la du amplifiloj kontraŭ la enigo: la deklivo estas la gajno, negativa por la inversa ŝtupo](fig:transfer)

La skemo mem estas la formularo. Ĝiaj kampoj estas la amplekso **Vin** (sub la enigo), la amplekso **Vout** (sub la eliro), $R_1$ kaj $R_2$ (super la rezistiloj) kaj la gajno **G** (ene de la amplifilo). Plenigu **sufiĉe** el ili kaj la resto estas kalkulata; la kalkulitaj kampoj ŝlosiĝas kaj iliaj valoroj aperas en la rezulta tabelo.

| Vi enmetas                                       | Vi ricevas                                      |
|--------------------------------------------------|-------------------------------------------------|
| $R_1$, $R_2$, amplekso de Vin                    | $G$, amplekso de Vout                           |
| $G$, $R_1$ (aŭ $R_2$), amplekso de Vin           | la alia rezistilo, amplekso de Vout             |
| amplekso de Vin, amplekso de Vout, $R_1$         | $G$, $R_2$                                      |
| amplekso de Vout, $G$                            | amplekso de Vin                                 |
| sola valoro de Vin kaj de Vout                   | $G$ (poste aldonu rezistilon por ricevi la alian) |

Notoj pri la enmeto:

- Amplekso estas tajpata kiel `min; maks`; unu nombro estas unu valoro (vidu *Amplekso* en *Rapida komenco*).
- Por la **inversa** amplifilo la gajno estas negativa, do la *minimuma* enigo donas la *maksimuman* eliron. La paĝo paras la finojn mem: $V_{in}$ = `-1; 1` kaj $G = -5$ donas $V_{out}$ = −5…5 V. Pozitiva $G$ estas rifuzata (*G devas esti negativa*).
- Por la **neinversa** amplifilo $G$ devas esti pli granda ol 1.
- Tensioj kaj la gajno estas precizaj nombroj. **Rezistiloj akceptas toleremojn**, ekzemple `10k 1%`. Enmeti $R_1$, $R_2$ kaj $G$ kune estas permesate: la gajno tiam estas prenata de la rezistiloj.

La rezulta tabelo listigas **Vin**, **Vout, ideala** (kun nominalaj rezistiloj) kaj, kiam ekzistas toleremoj, **Vout kun toleremoj** (la plej malalta kaj la plej alta eliro, kiun la cirkvito povas doni), poste $R_1$, $R_2$ kaj $G$ kun ilia disvastiĝo. La langeto **Grafikaĵo** montras $V_{out}(V_{in})$; kun toleremoj ĝi aldonas la liniojn de la plej granda kaj la plej malgranda gajno, la bando, en kiu kuŝas la reala eliro.

## Tolero de la gajno

La gajno dependas de la *proporcio* de rezistiloj, do la tolero de rezistiloj iras rekte en la precizecon de la ŝtupo. En la inversa amplifilo la gajno estas plej granda, kiam la reaga rezistilo estas ĉe sia maksimumo kaj la enira rezistilo ĉe sia minimumo:

$$\lvert G \rvert_{max} = \frac{R_2 (1 + t)}{R_1 (1 - t)} \approx \lvert G \rvert\,(1 + 2t)$$

Por 1 % rezistiloj la gajno estas necerta je ĉirkaŭ ±2 %; kun 0.1 % partoj je ±0.2 %. En la neinversa ŝtupo la eraro estas pli malgranda, ĉar $G = 1 + R_1/R_2$ enhavas precizan 1: por $G = 11$ la relativa eraro estas ĉirkaŭ $2t \cdot 10/11$.

## Laborita ekzemplo: skalado de sensilo

Sensilo donas −1 V ĝis +1 V kaj la ADC akceptas −5 V ĝis +5 V.

1. **Gajno.** Malfermu **OA (inversa)** kaj enmetu $V_{in}$ = `-1; 1` kaj $V_{out}$ = `-5; 5`. La paĝo trovas $G = -5$.
2. **Rezistiloj.** Aldonu $R_1$ = `10k`: la paĝo redonas $R_2 = 50\ \text{k}\Omega$. Tio ne estas norma valoro; la plej proksima valoro de E96 estas 49.9 kΩ.
3. **Realaj partoj.** Forviŝu $V_{out}$ kaj enmetu $R_1$ = `10k 1%`, $R_2$ = `49.9k 1%`. Nun $G = -4.99$ nominale kaj inter −5.091 kaj −4.891 (±2 %) en la plej malbona kazo, kaj la eliro atingas ±5.091 V. Se la ADC ne rajtas vidi pli ol ±5 V, malaltigu la gajnon iomete aŭ uzu 0.1 % rezistilojn.

Se vi bezonas pozitivan gajnon kaj altan eniran reziston, elektu **OA (neinversa)**: kun $R_1 = 50\ \text{k}\Omega$ kaj $R_2 = 10\ \text{k}\Omega$ ($G = 6$) enigo de 0…2 V fariĝas 0…12 V.

# Diferenca amplifilo

**Kategorio:** Analogaj. **Fonto:** TI, *Difference amplifier (subtractor) circuit*, [SBOA274](https://www.ti.com/lit/SBOA274).

Diferenca amplifilo plifortigas la **diferencon** de du tensioj kaj ignoras tion, kion ili havas komune. Ĝi legas la tension trans kurent-senta rezistilo, kiu troviĝas for de tero, la eliron de ponta sensilo, aŭ signalon, kies tero ne estas la tero de la ADC.

## La cirkvito

![Diferenca amplifilo: R2 = R1 kaj R4 = R3](fig:differential)

$V_{i1}$ atingas la inversan enigon tra $R_1$, kun $R_3$ kiel reago; $V_{i2}$ atingas la neinversan enigon tra la dividilo $R_2$, $R_4$, kies malsupra fino iras al la referenco $V_{ref}$. Per **superpozicio**:

- $V_{i1}$ sole (la + enigo tenata fiksa) vidas inversan amplifilon: $-V_{i1} \dfrac{R_3}{R_1}$.
- $V_{i2}$ kaj $V_{ref}$ starigas la + enigon al $V_+ = \dfrac{V_{i2} R_4 + V_{ref} R_2}{R_2 + R_4}$, kiun la ŝtupo plifortigas kiel neinversa amplifilo: $V_+ \left( 1 + \dfrac{R_3}{R_1} \right)$.

$$V_{out} = -V_{i1} \frac{R_3}{R_1} + \frac{V_{i2} R_4 + V_{ref} R_2}{R_2 + R_4} \left( 1 + \frac{R_3}{R_1} \right)$$

Kun **akordigitaj paroj** $R_2 = R_1$ kaj $R_4 = R_3$ tio kolapsas al

$$V_{out} = (V_{i2} - V_{i1}) \cdot G + V_{ref}, \qquad G = \frac{R_3}{R_1}$$

Tio, kion la du enigoj havas komune, nuliĝas, kaj $V_{ref}$ estas aldonata al la eliro senŝanĝe. Referenco ĉe la mezo de la nutrado movas dupolusan diferencon en la amplekson de unu-nutra ADC.

## Enira rezisto kaj la referenco

- La du enigoj ŝarĝas siajn fontojn malsame: $V_{i1}$ vidas $R_1$ (al nodo, kiu moviĝas kun la signalo), $V_{i2}$ vidas $R_2 + R_4$. Inter la enigoj la rezisto estas $R_1 + R_2$.
- **Fonta rezisto aldoniĝas al $R_1$ kaj $R_2$** kaj do ŝanĝas la gajnon kaj la ekvilibron de la paroj. Pelu la enigojn el malalt-impedancaj fontoj, aŭ bufru ĉiun enigon: tio estas la *instrumenta amplifilo*.
- **La referenco devas esti firma.** Ĝia fonta rezisto aldoniĝas al $R_4$. Uzu referencan IC aŭ bufritan dividilon, ne nudan rezistilan dividilon.

## La paĝo

La kampoj sidas sur la skemo: la diferenca amplekso **Vin diff** sub la enigoj, la amplekso **Vout** sub la eliro, **G** ene de la amplifilo, $R_1$ kaj $R_3$ super siaj rezistiloj kaj **Vref** ĉe la referenca terminalo. $R_2 = R_1$ kaj $R_4 = R_3$ estas implicitaj.

- **Ajnaj du el Vin diff, Vout kaj G** donas la trian ($V_{out} = V_{diff} \cdot G + V_{ref}$). $R_3$ kune kun $R_1$ ankaŭ fiksas la gajnon.
- Unu rezistilo kaj la gajno donas la alian: $R_3 = G \cdot R_1$, aŭ $R_1 = R_3 / G$.
- $V_{ref}$ estas postulata. Unu nombro en amplekso-kampo signifas la amplekson de 0 ĝis ĝi.
- $V_{ref}$, $R_1$ kaj $R_3$ akceptas toleremojn. Ĉiu el la kvar rezistiloj tiam povas troviĝi ie ajn en sia amplekso, sendepende (kalkulita rezistilo prenas la toleremon de la alia), do la tabelo montras la veran plej malbonan kazon de reala cirkvitplato, malakordon inkluzive.

La rezulta tabelo listigas $R_1 = R_2$, $R_3 = R_4$, Vin, Vout (ideala kaj kun toleremoj) kaj $G$. La grafikaĵo montras la eliron kontraŭ la diferenco, la nivelon de $V_{ref}$ kaj, kun toleremoj, la bandon de la eliro.

## Laborita ekzemplo: la projekto SBOA274

Celo: diferenco de ±1.25 V fariĝu ±2.5 V, $V_{ref} = 0$.

1. **Gajno.** Enmetu Vin diff = `-1.25; 1.25`, Vout = `-2.5; 2.5` kaj Vref = `0`. $G = 5 / 2.5 = 2$.
2. **Rezistiloj.** Enmetu $R_1$ = `10k`: $R_3 = G \cdot R_1 = 20\ \text{k}\Omega$. Ambaŭ estas normaj valoroj.
3. **Tolero.** Enmetu $R_1$ = `10k 1%`, $R_3$ = `20k 1%`: $G$ estas 1.960…2.040 (±2 %) kaj la eliro atingas ±2.551 V. Kun 0.1 % rezistiloj la gajna eraro falas al ±0.2 %.

**Unu nutro.** Por 3.3 V ADC ŝovu la eliron al la mezo de la nutrado: Vin diff = `-0.8; 0.8`, $G = 2$, $V_{ref} = 1.65$ V donas $V_{out}$ = 0.05…3.25 V. Kun 0.1 % rezistiloj la eliro restas inter 46.7 mV kaj 3.256 V.

# Nivelŝoviloj

**Kategorio:** Analogaj. Du kalkuliloj, **Nivelŝovilo (inversa)** kaj **Nivelŝovilo (neinversa)**, projektas la klasikan interfacon inter sensilo kaj ADC: amplekso, kiu ne konvenas al la konvertilo, estas skalata *kaj ŝovata* sur ĝian enigan amplekson.

## Gajno kaj ŝovo

Nivelŝovilo bildigas la enigan amplekson $V_{in,min} \dots V_{in,max}$ sur la eligan amplekson $V_{out,min} \dots V_{out,max}$ per rekta linio:

$$V_{out} = G \cdot V_{in} + V_{ŝovo}, \qquad \lvert G \rvert = \frac{V_{out,max} - V_{out,min}}{V_{in,max} - V_{in,min}}$$

La gajno venas de la du larĝoj; la ŝovo de ajna unu paro de respondaj finoj. Simpla amplifilo donas la ĝustan deklivon tra nulo; **referenca tensio** donita tra rezistilo movas la linion supren aŭ malsupren je la ŝovo.

La du kalkuliloj sekvas du projektojn de la serio *Analog Engineer's Circuit* de TI:

| Kalkulilo | Dokumento | Signalo eniras | Referenco eniras |
|---|---|---|---|
| Nivelŝovilo (inversa) | [SBOA261](https://www.ti.com/lit/SBOA261) | inversa enigo | inversa enigo |
| Nivelŝovilo (neinversa) | [SBOA262](https://www.ti.com/lit/SBOA262) | neinversa enigo | inversa enigo |

En ambaŭ la referenco eniras la **inversan** enigon. Referenco tie kontribuas kun minusa signo: pozitiva $V_{ref}$ movas la eliron **malsupren**, negativa movas ĝin supren. En ĉi tiuj ĉapitroj, kiel en la TI-dokumentoj, $R_1$ estas la reaga rezistilo.

## Inversa nivelŝovilo

![Inversa nivelŝovilo (SBOA261): Vin tra R2 kaj Vref tra R3 renkontiĝas ĉe la inversa enigo, R1 estas la reago](fig:shifter_inv)

La + enigo estas surterigita, do la − enigo estas virtuala tero. KCL tie: la kurentoj de $V_{in}$ tra $R_2$, de $V_{ref}$ tra $R_3$ kaj de la eliro tra $R_1$ sumiĝas al nulo:

$$\frac{V_{in}}{R_2} + \frac{V_{ref}}{R_3} + \frac{V_{out}}{R_1} = 0 \quad\Rightarrow\quad V_{out} = -V_{in} \frac{R_1}{R_2} - V_{ref} \frac{R_1}{R_3}$$

La ŝtupo inversigas: $V_{in,max}$ donas $V_{out,min}$. La projekto sekvas el la du termoj:

1. **Gajno.** $\lvert G \rvert = R_1/R_2$ el la larĝoj.
2. **Rezistiloj de la gajno.** Elektu $R_1$ (1–100 kΩ), poste $R_2 = R_1 / \lvert G \rvert$.
3. **Ŝovo.** Ĉe $V_{in,max}$ la eliro devas esti $V_{out,min}$: $V_{out,min} = -V_{in,max} \lvert G \rvert - V_{ref} R_1/R_3$, do

$$\frac{R_1}{R_3} = -\frac{V_{out,min} + \lvert G \rvert\, V_{in,max}}{V_{ref}}$$

4. **Referenca rezistilo.** $R_3 = R_1 / (R_1/R_3)$. Ĝi devas esti pozitiva: $V_{ref}$ bezonas la signon kontraŭan al $C = V_{out,min} + \lvert G \rvert V_{in,max}$. Se $C = 0$, ŝovo ne necesas kaj $R_3$ estas ellasata.
5. **Normaj valoroj.** Rondigu $R_2$ kaj $R_3$ kaj kontrolu la realan eliron.

![La gajno sole donas la ĝustan deklivon en la malĝusta loko; la referenco movas la linion en la fenestron de la du amplekso (ekzemplo SBOA261)](fig:shift_inv)

**Praktikaj punktoj.** La enira rezisto estas $R_2$: la fonto devas povi alimenti ĝin. La referenco alimentas $R_3$ kaj devas esti firma.

## Laborita ekzemplo: la projekto SBOA261

Celo: sensilo donas −5 V…−1 V, la ADC prenas 0.05 V…3.3 V, la referenco estas 5 V. Enmetu Vin = `-5; -1`, Vout = `0.05; 3.3`, Vref = `5`, $R_1$ = `845`.

1. $\lvert G \rvert = \dfrac{3.3 - 0.05}{-1 - (-5)} = \dfrac{3.25}{4} = 0.8125$ (la paĝo montras $G = -0.8125$).
2. $R_2 = 845\ \Omega / 0.8125 = 1.040\ \text{k}\Omega$, valoro de E192.
3. $\dfrac{R_1}{R_3} = -\dfrac{0.05 + 0.8125 \cdot (-1)}{5} = \dfrac{0.7625}{5} = 0.1525$.
4. $R_3 = 845\ \Omega / 0.1525 = 5.541\ \text{k}\Omega$; la plej proksima valoro de E192 estas 5.56 kΩ.
5. Enmetu $R_3$ = `5.56k`: la eliro fariĝas 52.6 mV…3.3026 V, 2.6 mV super la celo ĉe ambaŭ finoj, malpli ol 0.1 % de la larĝo.
6. Kun 1 % rezistiloj (`845 1%`, `1.04k 1%`, `5.56k 1%`) la eliro disvastiĝas sur 36.5 mV…3.385 V kaj la gajno je ±2 %: iomete preter la peta 0.05…3.3 V. 0.1 % rezistiloj tenas ĝin ene de ĉirkaŭ 0.2 %.

## Neinversa nivelŝovilo

![Neinversa nivelŝovilo (SBOA262): Vin tra la dividilo R3, R4 al la + enigo, Vref tra R2 al la − enigo, R1 estas la reago](fig:shifter_ni)

La enira dividilo starigas la + enigon al $V_+ = V_{in} \cdot K$ kun $K = R_4/(R_3 + R_4)$. KCL ĉe la − enigo, kiu egalas $V_+$, kun $V_{ref}$ tra $R_2$ kaj la eliro tra $R_1$:

$$\frac{V_{ref} - V_+}{R_2} + \frac{V_{out} - V_+}{R_1} = 0 \quad\Rightarrow\quad V_{out} = V_{in} \frac{R_4}{R_3 + R_4} \left( 1 + \frac{R_1}{R_2} \right) - V_{ref} \frac{R_1}{R_2}$$

La deklivo estas $G = K (1 + R_1/R_2)$, kaj la ŝovo estas $-V_{ref} R_1/R_2$. La projekto:

1. **Gajno.** $G$ el la larĝoj; ĉi tie $V_{in,min}$ donas $V_{out,min}$.
2. **Ŝovo.** Ĉe $V_{in,min}$: $V_{out,min} = G V_{in,min} - V_{ref} R_1/R_2$, do kun $D = G V_{in,min} - V_{out,min}$

$$\frac{R_1}{R_2} = \frac{D}{V_{ref}}, \qquad R_2 = R_1 \frac{V_{ref}}{D}$$

  $V_{ref}$ devas havi la signon de $D$. Se $D = 0$, nenio ŝoviĝas: uzu simplan neinversan amplifilon.
3. **Dividilo.** $K = G / (1 + R_1/R_2)$; elektu $R_4$, poste $R_3 = R_4 (1/K - 1)$.
4. **La limo.** Dividilo povas nur malfortigi, $K < 1$, do $1 + R_1/R_2 > G$. Kun $R_1/R_2 = D/V_{ref}$ tio limigas la referencon: por $G > 1$

$$\lvert V_{ref} \rvert < \frac{\lvert D \rvert}{G - 1}$$

  Super ĝi la paĝo raportas, ke $R_3$ ne estus pozitiva, kaj diras al vi la limon.

![La sama ideo por la neinversa ŝtupo (ekzemplo SBOA262)](fig:shift_ni)

**Praktikaj punktoj.** La enira rezisto estas $R_3 + R_4$. La referenco alimentas $R_2$ kaj devas esti firma.

## Laborita ekzemplo: la projekto SBOA262

Celo: 2 V…5 V en 0.05 V…4.95 V kun 2.5 V referenco, $R_1 = R_4 = 1\ \text{k}\Omega$. Enmetu Vin = `2; 5`, Vout = `0.05; 4.95`, Vref = `2.5`, $R_1$ = `1k`, $R_4$ = `1k`.

1. $G = \dfrac{4.95 - 0.05}{5 - 2} = 1.6333$.
2. $D = 1.6333 \cdot 2 - 0.05 = 3.2167$ V; $R_1/R_2 = 3.2167 / 2.5 = 1.2867$; $R_2 = 1\ \text{k}\Omega / 1.2867 = 777.2\ \Omega$ → E192: 777 Ω.
3. $K = 1.6333 / 2.2867 = 0.7143$; $R_3 = 1\ \text{k}\Omega \cdot (1/0.7143 - 1) = 400\ \Omega$ → E96: 402 Ω.
4. Enmetu $R_2$ = `777`, $R_3$ = `402`: la eliro estas 45.0 mV…4.939 V kaj la gajno 1.6312.
5. La referenco povas iri ĝis $3.2167 / 0.6333 = 5.08$ V; 2.5 V estas ĉirkaŭ duono de tio, komforta elekto.
6. Kun 1 % rezistiloj ĉie la eliro disvastiĝas sur −1.9 mV…5.014 V, multe preter la peta 0.05…4.95 V: 1 % partoj ne sufiĉas ĉi tie, uzu 0.1 % rezistilojn.

## Elekto de la referenco

La referenco starigas la ŝovon, do ĝi devas esti **preciza**, **stabila** kaj **firma** (malalta eliga impedanco):

- **Referenca IC** (serioj de referencoj de 1.25, 2.048, 2.5, 3.0, 3.3, 4.096, 5 V kaj pli) donas stabilan, malalt-impedancan tension.
- **La ADC-referenco** mem faras la mezuradon *proporcia*: la eraro de la referenco tiam nuliĝas en la legado.
- Nuda rezistila dividilo ne estas firma: ĝia fonta rezisto aldoniĝas al $R_3$ (inversa) aŭ $R_2$ (neinversa). Bufru ĝin per sekvilo.

**Kiam la kampo Vref estas malplena**, la paĝo elektas valoron el la du amplekso:

- Se la donitaj rezistiloj jam fiksas ĝin (inversa: $R_3$ kun $R_1$ aŭ $R_2$; neinversa: $R_1$ kun $R_2$), la paĝo kalkulas la **postulatan** referencon: $V_{ref} = -C R_3/R_1$, aŭ $V_{ref} = D R_2/R_1$.
- Alie ĝi **rekomendas** unu el la normaj valoroj 1.024, 1.25, 2.048, 2.5, 3.0, 3.3, 4.096, 5 kaj 10 V, kun la signo, kiun la ŝovo bezonas. Por la inversa ŝtupo ĝi prenas la valoron plej proksiman al $\lvert C \rvert$, tiel ke $R_3$ rezultas proksima al $R_1$. Por la neinversa ŝtupo ĝi prenas la valoron plej proksiman al duono de la limo $\lvert D \rvert/(G - 1)$, sekure sub ĝi.
- Se neniu ŝovo necesas, la inversa paĝo diras tion kaj lasas $R_3$ malfermita.

La valoro aperas kiel griza sugesto de la kampo Vref kaj kiel vico de la rezulta tabelo. Tajpu ĝin, aŭ vian propran, en la kampon. Por la ekzemplo SBOA261 la rekomendo estas 1.024 V kun $R_3 = 1.135\ \text{k}\Omega$; TI uzas la 5 V nutradon kiel referencon, kio ankaŭ funkcias, kun pli granda $R_3$. Por SBOA262 la rekomendo estas la 2.5 V de la dokumento.

## La paĝoj de Nivelŝoviloj

La kampoj sidas sur la skemo: la amplekso **Vin** kaj **Vout**, **G**, **Vref** kaj la rezistiloj.

- **Ambaŭ amplekso** kun $V_{ref}$ (donita aŭ elektita) fiksas la projekton. La inversa paĝo tiam bezonas **unu rezistilon**, $R_1$, $R_2$ aŭ $R_3$, kaj kalkulas la aliajn. La neinversa paĝo bezonas **unu rezistilon el ĉiu paro**: $R_1$ aŭ $R_2$, kaj $R_3$ aŭ $R_4$.
- **Unu amplekso** sufiĉas, kiam la rezistiloj fiksas la reston. Inversa: $R_3$ kaj du el $R_1$, $R_2$, $G$. Neinversa: $R_1$, $R_2$ kaj aŭ $G$ aŭ $R_3$ kun $R_4$.
- Tajpita rezistilo estas uzata kiel tajpita: enmetu normajn valorojn en la kalkulitajn kampojn por vidi la realan eliron.
- $V_{ref}$ kaj la rezistiloj akceptas toleremojn; kalkulita rezistilo prenas la toleremon de la donita, de kiu ĝi sekvas.

La rezulta tabelo listigas la rezistilojn, $V_{ref}$ (donita, postulata aŭ rekomendata), Vin, Vout (ideala kaj kun toleremoj) kaj $G$. La grafikaĵo montras la **postulatan** linion, la linion **kun ĉi tiuj rezistiloj**, la nivelon de $V_{ref}$ kaj, kun toleremoj, la bandon de la eliro.

La paĝo montras ruĝan mesaĝon, kiam la amplekso ne povas esti bildigitaj: referenco kun malĝusta signo (kun la valoro de la ĝusta signo por uzi), referenco super la limo de la neinversa ŝtupo, amplekso kun la malsupra fino super la supra.

# Referenco

## SI-prefiksoj akceptataj en la enigo

La prefiksoj estas tiuj de la Internacia Sistemo de Unuoj (BIPM, *The International System of Units*, la SI-Broŝuro). La enigo uzas `u` por mikro (µ).

| Simbolo | Nomo  | Faktoro     | Ekzemplo          |
|---------|-------|-------------|-------------------|
| `G`     | giga  | $10^{9}$    | `1G` = 1 000 000 000 |
| `M`     | mega  | $10^{6}$    | `2.2M`            |
| `k`     | kilo  | $10^{3}$    | `4.7k`            |
| (nenio) | —     | $1$         | `100`             |
| `m`     | milli | $10^{-3}$   | `500m`            |
| `u`     | mikro | $10^{-6}$   | `10u`             |
| `n`     | nano  | $10^{-9}$   | `100n`            |
| `p`     | piko  | $10^{-12}$  | `22p`             |

Rezultoj estas presataj kun la plej proksima inĝenieria prefikso (oblo de 1000), kun la nombro de decimaloj starigita en Agordoj.

## Formula ŝtelfolio

| Temo                      | Formulo                                                          |
|---------------------------|------------------------------------------------------------------|
| Leĝo de Ohm               | $U = I R$                                                        |
| Potenco                   | $P = U I = I^2 R = U^2 / R$                                      |
| Seria / paralela          | $R = R_1 + R_2$ / $R = R_1 R_2 / (R_1 + R_2)$                    |
| Tensiodividilo            | $V_{out} = V_{in} R_2 / (R_1 + R_2)$                             |
| Tolero de la dividila proporcio | $\Delta k / k \approx 2t(1 - k)$                           |
| Rezisto de drato          | $R = \rho L / A$,  $\rho_{Cu} = 1.72 \cdot 10^{-8}\ \Omega\text{m}$ |
| Diametro de AWG           | $d = 0.127\ \text{mm} \cdot 92^{(36 - n)/39}$                    |
| Temperaturaltiĝo          | $\Delta T = P R_{th}$                                            |
| Plenigfaktoro             | $D = t_{on} / T$                                                 |
| RMS de rektangulo         | $I_p \sqrt{D}$                                                   |
| RMS de duonsinuso         | $I_p \sqrt{D/2}$                                                 |
| RMS de triangulo          | $I_p \sqrt{D/3}$                                                 |
| Inversa amplifilo         | $G = -R_2 / R_1$                                                 |
| Neinversa amplifilo       | $G = 1 + R_1 / R_2$                                              |
| Diferenca amplifilo       | $V_{out} = (V_{i2} - V_{i1}) R_3/R_1 + V_{ref}$                  |
| Inversa nivelŝovilo       | $V_{out} = -V_{in} R_1/R_2 - V_{ref} R_1/R_3$                    |
| Neinversa nivelŝovilo     | $V_{out} = V_{in} \frac{R_4}{R_3 + R_4}(1 + \frac{R_1}{R_2}) - V_{ref} \frac{R_1}{R_2}$ |

## Normoj

| Normo        | Temo                                                                    |
|--------------|-------------------------------------------------------------------------|
| IEC 60063    | Vicoj de preferataj nombroj por rezistiloj kaj kondensiloj: E6 … E192   |
| IEC 60062    | Markaj kodoj por rezistiloj kaj kondensiloj (kolorbendoj, literkodoj)   |
| IEC 60028    | Internacia normo de rezisto por kupro (kalcinita kupro)                 |
| IEC 60228    | Konduktoroj de izolitaj kabloj: metrikaj sekcoj en mm²                  |
| ASTM B258    | Nominalaj diametroj kaj sekcoj de AWG-mezuroj de ronda drato            |
| SI-Broŝuro   | BIPM, *The International System of Units*: unuoj kaj prefiksoj          |

## Aplikaj notoj

La ĉapitroj pri OA sekvas ĉi tiujn dokumentojn; ili estas senpage elŝuteblaj.

- Texas Instruments, *Inverting op amp with inverting positive reference voltage circuit*, [SBOA261](https://www.ti.com/lit/SBOA261). La projekto de **Nivelŝovilo (inversa)**.
- Texas Instruments, *Non-inverting op amp with inverting positive reference voltage circuit*, [SBOA262](https://www.ti.com/lit/SBOA262). La projekto de **Nivelŝovilo (neinversa)**.
- Texas Instruments, *Difference amplifier (subtractor) circuit*, [SBOA274](https://www.ti.com/lit/SBOA274). La projekto de **OA (diferenca)**.
- Texas Instruments, B. Carter, R. Mancini, *Op Amps for Everyone*, [SLOD006](https://www.ti.com/lit/SLOD006): kompleta projekta gvidilo pri cirkvitoj kun OA.

## Glosaro

- **Nominala valoro**: la valoro, kiun komponanto aŭ rezulto havas sen deviiĝo; la kolumno *Nom*.
- **Tolero**: la permesata deviiĝo de komponanto de ĝia nominala valoro, en procentoj (`10k 1%`).
- **Plej malbona kazo**: la plej malfavora kombino de la toleremoj de ĉiuj komponantoj; la kolumnoj *Min* kaj *Maks*.
- **Δ, %**: kiom for de la nominala valoro kuŝas Min kaj Maks, en procentoj.
- **Intervala aritmetiko**: kalkulado kun amplekso (nominala, minimuma, maksimuma) tiel, ke la rezulto enhavas ĉiun eblan rezulton.
- **Angula analizo**: trovo de la plej malbona kazo per kalkulo de cirkvito ĉe ĉiu kombino de la minimumo kaj maksimumo de ĝiaj enigoj.
- **SI-prefikso**: la litero post nombro, kiu skalas ĝin per potenco de 1000: `k`, `M`, `m`, `u`, `n`…
- **Kalkulita kampo**: kampo, kiun la programo plenigas el la aliaj; ĝi estas ŝlosita, ĝis vi forviŝas unu el viaj enigoj.
- **Amplekso**: la larĝo de signalo, tajpata kiel `min; maks`.
- **Leĝoj de Kirchhoff**: la kurentoj en nodon sumiĝas al nulo (KCL); la tensioj ĉirkaŭ buklo sumiĝas al nulo (KVL).
- **Superpozicio**: en lineara cirkvito la respondo al pluraj fontoj estas la sumo de la respondoj al ĉiu fonto aparte.
- **E-vico**: la preferataj valoroj de komponantoj, E6 ĝis E192 (IEC 60063).
- **AWG**: American Wire Gauge, la numero de dratmezuro; pli granda numero signifas pli maldikan dratmezuron.
- **Termika rezisto**: la temperaturaltiĝo por vato de disipata potenco, en K/W.
- **RMS**: radiko de meza kvadrato, la efektiva valoro de varianta grando; por kurento, la kontinua valoro kun la sama hejta efiko.
- **Plenigfaktoro**: la frakcio de la periodo, en kiu impulso estas ŝaltita.
- **Pika faktoro / formfaktoro**: piko super RMS, kaj RMS super meza, de ondoformo.
- **Virtuala fuŝkontakto / virtuala tero**: la kondiĉo en reaga buklo, kiu egaligas la du enigojn de OA, aŭ la inversan enigon kun la tero.
- **Gajno (G)**: la proporcio de eliga al eniga ŝanĝo, en V/V; negativa por inversa ŝtupo.
- **Ŝovo**: la movo, kiun nivelŝovilo aldonas al la plifortigita signalo, starigita de la referenco.
- **Nivelŝovilo**: amplifilo kun referenco, kiu bildigas unu tensian amplekson sur alian, kun gajno kaj ŝovo.
- **Referenco (Vref)**: preciza, stabila, malalt-impedanca tensio, kiu starigas ŝovon. Sur la paĝoj de nivelŝoviloj ĝi estas *donita*, *postulata* (fiksita de la donitaj rezistiloj) aŭ *rekomendata* (elektita de la programo).
- **Proporcia (ratiometria)**: mezurado, kies rezulto dependas de proporcio de tensioj, tiel ke la eraro de komuna referenco nuliĝas.

## Mesaĝoj, kiujn vi povas vidi

| Mesaĝo                                     | Kion fari                                                        |
|--------------------------------------------|------------------------------------------------------------------|
| Korektu la erarojn en la kampoj            | kampo havas ruĝan randon; legu ĝian mesaĝon (ŝvebu sur skemo)    |
| Donu 3 el 4 grandoj                        | plenigu pli da kampoj (la nombro de donitaj estas montrata)      |
| Donite 3 — necesas ĝuste 2                 | forviŝu kampon: la kalkulilo prenas ĝuste tiom da enigoj         |
| Vout1 devas esti pli malgranda ol Vin      | la tensioj devas esti en la ordo de la dividilo                  |
| G devas esti negativa / pli granda ol 1    | la signo aŭ grando de la gajno ne konvenas al la tipo de amplifilo |
| Donu Vin kaj Vout kiel "min; maks" …       | la malsupra fino de amplekso devas veni unue                     |
| Plenigu du el: Vindiff, Vout, G            | la diferenca amplifilo bezonas du el la tri                      |
| Plenigu Vref, aŭ ambaŭ Vin kaj Vout …      | donu la referencon, aŭ ambaŭ amplekso por ke ĝi estu rekomendata |
| Vref … havas malĝustan signon por tiu ŝovo | uzu referencon kun kontraŭa signo (la valoro estas sugestata)    |
| R3 ne estus pozitiva …                     | la referenco estas tro granda por la neinversa nivelŝovilo: malaltigu ĝin sub la montrata limo |
| Vin min·G egalas Vout min …                | neniu ŝovo necesas: uzu OA (neinversa)                           |
| la daŭro de la impulso ne devas superi la periodon | $t_{on}$ estas pli longa ol $T$                          |
| t1 + t2 + t3 devas egali la periodon T     | forviŝu unu el la kvar tempoj                                    |

## Plia legado

- Texas Instruments, *Analog Engineer's Circuit Cookbook: Amplifiers*: la kolekto de cirkvitoj, de kie venas la paĝoj de OA.
- P. Horowitz, W. Hill, *The Art of Electronics*: la klasika praktika kurso.
- A. Sedra, K. Smith, *Microelectronic Circuits*: detala universitata teksto pri amplifiloj.
- B. Carter, R. Mancini, *Op Amps for Everyone*: la projekta gvidilo de TI (vidu *Aplikaj notoj*).
- La datumfolio de via komponanto: ĉiam la lasta vorto pri limoj kaj toleremoj.

# Pri la programo

| Kampo     | Valoro                                                   |
|-----------|----------------------------------------------------------|
| Nomo      | ECW — Engineers' Circuit Workbench                       |
| Devizo    | Rapidaj kalkuloj. Praktika inĝenierarto.                 |
| Versio    | 0.2.0                                                    |
| Licenco   | MIT                                                      |
| Platformo | Tauri 2, kerno en Rust, interfaco en Preact              |
| Fonto     | [github.com/zvladimir/ecw](https://github.com/zvladimir/ecw) |
| Subteno   | [boosty.to/desialsi](https://boosty.to/desialsi)         |

ECW estas libera programaro. Ĉu vi trovis malĝustan rezulton? Bonvolu raporti ĝin en GitHub kun la valoroj, kiujn vi enmetis, tio, kion vi atendis, kaj la fonto de la formulo.

© 2024–2026 Zibtsev Vladimir
