package com.JP_Studios.DeclarationClasses.Comparator;

import com.JP_Studios.Schueler;

import java.util.Comparator;

/**
 * {@link Comparator} zum Sortieren der Schüler nach MiMiMiPunktzahl
 */
public class SchuelerPunktzahlComparator implements Comparator<Schueler> {
    @Override
    public int compare(Schueler s1, Schueler s2) {
        return Integer.compare(s2.punktzahl, s1.punktzahl);
    }
}
