# IEEM: Südafrika Datenübertragung

Basis der Entwicklung soll das LARthon sein; es sollen robuste und stabile Teile verwendet werden. Hergestellt werden sollen 6 Stück in Gehäuse (etwa 20 x 10 x 10 cm).

Aufbau dieser Daten -sammel-, -verarbeitung-, und -übertragungs- Station
Zentrale Einheit besteht aus 2 parallel angeordneten Platinenstapeln (basierend auf Rasperry Pi mit Lan Ausgang und Display) – siehe Bild 1 -, einen Analog-Signal Konverter,  vor-Ort gekaufter Router mit SIM card, Stromversorgung (aus consumer Bereich, Überspannungsschutz)

## Datenspeicher Micro-SD

## die Platinenstapel

 sollen einfach ausgetauscht werden können Datenspeicherung und -verarbeitung

## Dateneingänge für 4 verschiedene Gerätetypen

NitriTox: Toxizität, Nährlösungsmenge. Ausgabe RS232, Ethernet (?)
Die Toxizitätsmessung ist zeitlich die längste, LAR Software gibt Format vor.
JUMO Elektroden: pH, EC, ORP, DO. Ausgabe RS232, Ethernet
Die Elektroden sind an die „JUMO AQUIS touch S“ Auswerteeinheit angeschlossen die eine sehr hohe Flexibilität der Messdaten verfügt, was Datenaufnahme der Elektroden in die Box und auch die Ausgabe an die zu entwickelnde Einheit betrifft. Manual wichtig für Optimierung. Default Wertwert-Übertragungsfrequenz 30 sec.
KIT Trübung: Eigenbau. Ausgabe Analog 0-20 mA, 0-5V (?)
Einen Reserve Platz für Erweiterungsmöglichkeit
erst bei Bedarf Möglichkeit in String einbinden können

## Redundanz

die Daten werden auf beiden Rasperry Pi parallel aufgenommen und verarbeitet ().

## Einrichtung einer Konfigurationsmaske

voreingestellt, veränderbar lokal (Kabel und Laptop) an der zu entwickelnden Einheit und extern über idealerweise funktionierenden Wlan Anschluss

## Vereinheitlichung der Datenfrequenzen

die Geräte liefern in unterschiedlichen Zeiten ihre Daten
gesendete Datensätze sollen konfigurierbar sein zwischen 10 – 120 min, default 20 min
wenn mehr Daten in der eingestellten Zeit von den Geräten geliefert werden, spezifische Datenverarbeitung vorsehen, wie Ausreissertest oder Mittelwertbildung

## Bildung eines Datenstrings

genaues Fomat hom Server ab,dieses Projekt nimmt ein Format für einen Entwicklungs-Server, wie beispielsweise Firebase von Google. Anpassung auf andere Server, beispielsweise Disy ist ein extra Zusatzprojekt (geringer Aufwand)
String enthält (neben einem Ort pro Dateneinheit): Datum, Urzeit, und (pro Parameter) Name, Einheit, Wert, Gerätestatus (beisielsweise „1“ für Gerät liefert einen gültigen Wert und „0“ für Gerät liefert keinen Wert)

## Speicherung der Strings auf der Micro-SD.

Wunsch: 1 Jahr. Optional für eine Erweiterung: Speicherung von Rohwerten. Daten Ausgabe

## Anzeige der Werte auf einem Display

jeweils die Werte des letzten erzeugten Strings, mehere (3) Diplay Seiten anzeigbar, umschaltbar mit einem Druckknopf 
pro Display Seite: vier Parameter mit Name, Einheit, Wert. Auch für Service wichtige Informationen, wie beispielsweise IP der angeschlossenen Routers
zur Zeit sind 7 Parameter auf der Liste

## lokale Ausgabe der Werte der Micro-SD über USB-Stick oder angeschlossenem Laptop (Lan)

## Datenübertragung

die erste Stufe dieses Projekts endet an der Ausgabe der Werte vom Rasberry Pi über Lan, die zweite aus einem in Deutschland aufgebauten Teststand aus zu entwickelnder Datenstation, Router und Datenbank (Cloud).
prüft Internet: wenn ja, wegschicken, wenn nein warten und alle nicht versendeten strings wegschicken.
Befehlsempfang
es soll in einer optionalen Stufe dieses Projektes geprüft werden, ob die in den Geräten vorhandendenen Möglichkeiten eines Befehlsempfang für die Bedienung der Stationen Sinn machen. Bei NitriTox können beispielsweise einzelne Messungen gestartet, gestoppt und verschoben werden, bei JUMO gibt es noch mehr, sehr vielfältige Möglichkeiten (auch Steuerung, Reglung), bei KIT-Trübung natürlich nichts.
