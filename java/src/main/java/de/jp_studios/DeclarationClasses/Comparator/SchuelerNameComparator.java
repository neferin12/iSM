package de.jp_studios.DeclarationClasses.Comparator;

import de.jp_studios.Schueler;

import java.util.Comparator;

/**
 * {@link Comparator} zum Sortieren der Schüler nach Name (alphabetisch)
 */
public class SchuelerNameComparator implements Comparator<Schueler> {

    @Override
    public int compare(Schueler o1, Schueler o2) {
        return o1.name.compareTo(o2.name);
    }
}
