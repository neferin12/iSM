package com.JP_Studios.DeclarationClasses;

import com.JP_Studios.Verteiler;

import java.util.Comparator;

/**
 * Created by Julian Pollinger
 */
public class VerteilerComparator implements Comparator<Verteiler> {
    @Override
    public int compare(Verteiler v1, Verteiler v2) {
        return Integer.compare(v1.punktzahl, v2.punktzahl);
    }
}
