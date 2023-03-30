package de.jp_studios.DeclarationClasses.Comparator;

import de.jp_studios.Verteiler;

import java.util.Comparator;

/**
 * {@link Comparator} zum Sortieren der Verteiler nach Gesamt-MiMiMiPunktzahl
 */
public class VerteilerComparator implements Comparator<Verteiler> {
    @Override
    public int compare(Verteiler v1, Verteiler v2) {
        return Integer.compare(v1.punktzahl, v2.punktzahl);
    }
}
