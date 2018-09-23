package com.JP_Studios;

import com.JP_Studios.DeclarationClasses.GlobalConstants;
import com.JP_Studios.DeclarationClasses.SchuelerPunktzahlComparator;
import com.JP_Studios.Exceptions.SchuelerLimitErreichtExeption;

import java.util.ArrayList;
import java.util.Collections;


public class Verteiler {
    private ArrayList<Schueler> schueler = new ArrayList<>();
    public int punktzahl;
    private ArrayList<Kurs>[] kurses = new ArrayList[2];
    ArrayList<Schueler> überschuss[] = new ArrayList[2];


    /**
     * @param schueler Die Schueler
     * @param kurse Die Kurs
     */
    public Verteiler(ArrayList<Schueler> schueler, ArrayList<Kurs>[] kurse) {
        for (Schueler schueler1 : schueler) {
            this.schueler.add(new Schueler(schueler1.name, schueler1.pseminarwahl, schueler1.wseminarwahl));
        }
        for (int i = 0; i < kurse.length; i++) {
            kurses[i] = new ArrayList<>();
            ArrayList<Kurs> seminar = kurse[i];
            for (Kurs kurs : seminar) {
                kurses[i].add(new Kurs(kurs.getName(), kurs.getpOw(), kurs.getMaximaleSchueler(), kurs.getIndex()));
            }

        }
        punktzahl = 0;
    }


    /**
     * Weißt die Schueler den Kursen zu
     * @return Gibt die {@link Schueler Schueler} zurueck, die keinen ihrer Wuensche bekommen haben. Das Array ist aufgeteilt in {@link GlobalConstants#W_SEMINAR} und {@link GlobalConstants#P_SEMINAR}
     */
    public ArrayList<Schueler>[] seminareVerteilen() {

        überschuss[0] = new ArrayList<>();
        überschuss[1] = new ArrayList<>();

        /**
         * P-Seminare
         */
        Collections.shuffle(schueler);
        for (Schueler schueler1 : schueler) {
            try {
                schueler1.kursSetzen(schueler1.pseminarwahl[0],GlobalConstants.P_SEMINAR,GlobalConstants.ERSTE_WAHL,false, this);
            } catch (SchuelerLimitErreichtExeption schuelerLimitErreichtExeption) {
                try {
                    schueler1.kursSetzen(schueler1.pseminarwahl[1],GlobalConstants.P_SEMINAR,GlobalConstants.ZWEITE_WAHL,false, this);
                } catch (SchuelerLimitErreichtExeption schuelerLimitErreichtExeption1) {
                    try {
                        schueler1.kursSetzen(schueler1.pseminarwahl[2],GlobalConstants.P_SEMINAR,GlobalConstants.DRITTE_WAHL,false, this);
                    } catch (SchuelerLimitErreichtExeption schuelerLimitErreichtExeption2) {
                        schueler1.keineWahlBekommen();
                        überschuss[GlobalConstants.P_SEMINAR].add(schueler1);
                    }
                }
            }
        }
        Collections.shuffle(schueler);
        Collections.sort(schueler, new SchuelerPunktzahlComparator());

        /**
         * W-Seminare
         */
        for (Schueler schueler1 : schueler) {
            try {
                schueler1.kursSetzen(schueler1.wseminarwahl[0],GlobalConstants.W_SEMINAR,GlobalConstants.ERSTE_WAHL,false, this);
            } catch (SchuelerLimitErreichtExeption schuelerLimitErreichtExeption) {
                try {
                    schueler1.kursSetzen(schueler1.wseminarwahl[1],GlobalConstants.W_SEMINAR,GlobalConstants.ZWEITE_WAHL,false, this);
                } catch (SchuelerLimitErreichtExeption schuelerLimitErreichtExeption1) {
                    try {
                        schueler1.kursSetzen(schueler1.wseminarwahl[2],GlobalConstants.W_SEMINAR,GlobalConstants.DRITTE_WAHL,false, this);
                    } catch (SchuelerLimitErreichtExeption schuelerLimitErreichtExeption2) {
                        schueler1.keineWahlBekommen();
                        überschuss[GlobalConstants.W_SEMINAR].add(schueler1);
                    }
                }
            }
        }

        for (Schueler schueler1 : schueler) {
            punktzahl += schueler1.punktzahl;
        }

        return überschuss;
    }



    public ArrayList<Schueler> getSchueler() {
        return schueler;
    }

    public ArrayList<Kurs> getKurse(int i) {
        return kurses [i];
    }

    public ArrayList<Kurs>[] getKurse() {
        return kurses;
    }
}
