# UML-Parser
### Entwickler: Benedikt Wiest und Fabian Husemann


### Einleitung
Ziel des Projektes war es eine Software zu entwickeln, welche eine strukturierte Textdatei in ein UML-Diagramm umwandelt. Die Software sollte in der Programmiersprache Rust programmiert werden. 

### Umsetzung
Die Software wurde in der Programmiersprache Rust umgesetzt. Zur Erstellung von Bildern haben wir die Rust-Bibliothek imageproc der Entwickler PistonDevelopers eingesetzt. Die Bedienung der Software erfolgt über die Konsole, die Ausgabe erfolgt im JPG-Format. Grundlegendes Konzept ist eine modulare objektorientierte Programmierung

### Bedienung

##### Textuelle Strukturierung - Klassendiagramm

*   Die Textstruktur orientiert sich an XML, ist allerdings hinsichtlich Syntax frei von uns interpretiert
*   Gülting sind folgende Tags

    | Tag      | Schließendes Tag | Beschreibung                     | Gültig in Tag |
    |----------|------------------|----------------------------------|-----------|
    | Object   | /Object          | Öffnet ein neues Objekt          | /         |
    | Attribut | /Attribut        | Öffnet ein neues Attribut        | Object    |
    | Function | /Function        | Öffnet eine neuee Funktion        | Object    |
    | Relation | /Relation        | Beziehung zwischen zwei Objekten | /         |
    
* Gültig sind folgende Bezeichner
    *   Tag Object

    | Bezeichner | Beschreibung      | Gültige Werte |
    |------------|-------------------|---------------|
    | Name:      | Name des Objektes | String        |
    
    * Tag Attribut
    
    | Bezeichner | Beschreibung               | Gültige Werte |
    |------------|----------------------------|---------------|
    | Name:      | Bezeichnung des Attributs  | String        |
    | Typ:       | Typ der Variable           | String        |
    | Wert:      | Default Wert des Attributs | Typ           |
    
    * Tag Function
    
    | Bezeichner | Beschreibung                      | Gültige Werte |
    |------------|-----------------------------------|---------------|
    | Name:      | Bezeichnung der Funktion          | String        |
    | Parameter: | Name des Parameters und Datentyp  | String        |
    | Return     | Rückgabetyp und Wert der Funktion | String        |
    
    * Tag Relation
    
    | Bezeichner   | Beschreibung                                    | Gültige Werte |
    |--------------|-------------------------------------------------|---------------|
    | Description: | Bezeichnung der Relation                        | String        |
    | Typ:         | Typ der Relation                                | String        |
    | From:        | Name des Objektes bei dem die Beziehung startet | String        |
    | To:          | Name des Objektes bei dem die Beziehung endet   | String        |

Relationstypen können sein:
* Vererbung
* Kennt
* Abhängig
* Aggregation

##### Beispiel für die textuelle Strukturierung
```xml
Object
	Name: Tier
	Attribut
		Name: Alter
		Typ: Integer
	/Attribut	
	Attribut
		Name: Farbe
		Typ: Color
		Wert: RED
	/Attribut
	Attribut
		Name: Farbe
		Typ: Color
		Wert: RED
	/Attribut	
/Object
Object
	Name: Hund
	Attribut
		Name: Farbe
		Typ: Color
	/Attribut
	Attribut
		Name: Farbe
		Typ: Color
		Wert: RED
	/Attribut
	Attribut
		Name: Farbe
		Typ: Color
		Wert: WHITE
	/Attribut
	Function
		Name: bellen
		Parameter: lautstärke: Integer
		Return: void
	/Function
/Object
Object
	Name: Katze
	Attribut
		Name: Farbe
		Typ: Color
	/Attribut
	Function
		Name: bellen
		Parameter: lautstärke: Integer
		Return: void
	/Function
/Object
Relation
	Description: Ein Hund ist ein Tier
	Typ: vererbung
	From: Tier
	To: Hund
/Relation
```
##### Textuelle Strukturierung - Use Case-Diagramm

Die Textstruktur orientiert sich an XML, ist allerdings hinsichtlich Syntax frei von uns interpretiert
*   Gülting sind folgende Tags

    | Tag      | Schließendes Tag | Beschreibung                     | Gültig in Tag |
    |----------|------------------|----------------------------------|-----------|
    | Akteur   | /Akteur       | Öffnet einen neuen Akteur         | /         |
    | System | /System        | Öffnet das System zum Akteur        | /         |
    | Usecase | /Usecase        | Öffnet ein Use Case im System       | /         |

    
* Gültig sind folgende Bezeichner
    *   Tag Akteur

    | Bezeichner | Beschreibung      | Gültige Werte |
    |------------|-------------------|---------------|
    | Name:      | Name des Akteurs | String        |
    
    * Tag System
    
    | Bezeichner | Beschreibung               | Gültige Werte |
    |------------|----------------------------|---------------|
    | Name:      | Bezeichnung des Systems  | String        |
    
    * Tag Usecase
    
    | Bezeichner | Beschreibung               | Gültige Werte |
    |------------|----------------------------|---------------|
    | Name:      | Bescrheibung des Use Case  | String        |
    
```xml
Akteur
	Name: Kunde
/Akteur
System
	Name: Online-Shop
	Usecase
		Description: Artikel auswählen
	/Usecase	
	Usecase
		Description: Anschrift eingeben
	/Usecase	
	Usecase
		Description: Zahlungsmethode wählen
	/Usecase
/System
```


    
    
    
 
 
 
 

    


    
