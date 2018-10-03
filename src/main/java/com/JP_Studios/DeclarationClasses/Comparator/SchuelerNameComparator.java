package com.JP_Studios.DeclarationClasses.Comparator;

import com.JP_Studios.Schueler;

import java.util.Comparator;

/**
 * Created by Julian Pollinger
 */
public class SchuelerNameComparator implements Comparator<Schueler> {

    @Override
    public int compare(Schueler o1, Schueler o2) {
        return o1.name.compareTo(o2.name);
    }
}
