package com.JP_Studios;

import com.JP_Studios.DeclarationClasses.Comparator.SchuelerNameComparator;
import com.JP_Studios.DeclarationClasses.Comparator.SchuelerPunktzahlComparator;
import com.JP_Studios.DeclarationClasses.GlobalConstants;

import java.util.ArrayList;
import java.util.Collections;

/**
 * Mit dieser Klasse werden die Sch�ler auf die Seminare verteilt
 */
public class Verteiler {
    public int punktzahl;
    private ArrayList<Schueler> schueler = new ArrayList<>();
    private ArrayList<Kurs>[] kurses = new ArrayList[2];
    private ArrayList<Schueler>[] �berschuss = new ArrayList[2];


    /**
     * @param schueler Die Schueler
     * @param kurse    Die Kurs
     */
    Verteiler(ArrayList<Schueler> schueler, ArrayList<Kurs>[] kurse) {
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
     * Wei�t die Schueler den Kursen zu
     *
     * @return Gibt die {@link Schueler Schueler} zurueck, die keinen ihrer Wuensche bekommen haben. Das Array ist aufgeteilt in {@link GlobalConstants#W_SEMINAR} und {@link GlobalConstants#P_SEMINAR}
     */
    void seminareVerteilen() {

        �berschuss[0] = new ArrayList<>();
        �berschuss[1] = new ArrayList<>();

//        P-Seminare
        Collections.shuffle(schueler);
        for (Schueler schueler1 : schueler) {
            if (!schueler1.kursSetzen(schueler1.pseminarwahl[0], GlobalConstants.P_SEMINAR, GlobalConstants.ERSTE_WAHL, false, this)) {
                if (!schueler1.kursSetzen(schueler1.pseminarwahl[1], GlobalConstants.P_SEMINAR, GlobalConstants.ZWEITE_WAHL, false, this)) {
                    if (!schueler1.kursSetzen(schueler1.pseminarwahl[2], GlobalConstants.P_SEMINAR, GlobalConstants.DRITTE_WAHL, false, this)) {
                        schueler1.keineWahlBekommen();
                        �berschuss[GlobalConstants.P_SEMINAR].add(schueler1);
                    }
                }
            }
        }
        Collections.shuffle(schueler);
        schueler.sort(new SchuelerPunktzahlComparator());

//       W-Seminare
        for (Schueler schueler1 : schueler) {
            if (!schueler1.kursSetzen(schueler1.wseminarwahl[0], GlobalConstants.W_SEMINAR, GlobalConstants.ERSTE_WAHL, false, this)) {
                if (!schueler1.kursSetzen(schueler1.wseminarwahl[1], GlobalConstants.W_SEMINAR, GlobalConstants.ZWEITE_WAHL, false, this)) {
                    if (!schueler1.kursSetzen(schueler1.wseminarwahl[2], GlobalConstants.W_SEMINAR, GlobalConstants.DRITTE_WAHL, false, this)) {
                        schueler1.keineWahlBekommen();
                        �berschuss[GlobalConstants.W_SEMINAR].add(schueler1);
                    }
                }
            }
        }

        for (Schueler schueler1 : schueler) {
            punktzahl += schueler1.punktzahl;
        }

    }

    /**
     * Sortiert alle Sch�lerlisten dieses Verteilers und der Kurse des Verteilers nach Alphabet. Sollte aus Performancegr�nden so selten wie m�glich angewandt werden
     */

    public void sort() {
        schueler.sort(new SchuelerNameComparator());
        �berschuss[0].sort(new SchuelerNameComparator());
        �berschuss[1].sort(new SchuelerNameComparator());
        for (ArrayList<Kurs> kurs : kurses) {
            for (Kurs kur : kurs) {
                kur.sort();
            }

        }
    }


    /**
     * @return Gibt eine Liste der Sch�ler zur�ck
     */
    public ArrayList<Schueler> getSchueler() {
        return schueler;
    }

    /**
     * Gibt die P- oder W-Seminare zur�ck
     *
     * @param typ {@link GlobalConstants#W_SEMINAR} oder {@link GlobalConstants#P_SEMINAR}
     * @return Die entsprechenden Seminare
     */
    public ArrayList<Kurs> getKurse(int typ) {
        return kurses[typ];
    }

    /**
     * @return Gibt alle Kurse zur�ck
     */
    public ArrayList<Kurs>[] getKurse() {
        return kurses;
    }

    /**
     * @return Gibt die Sch�ler zur�ck, die kein Seminar bekommen haben
     */
    public ArrayList<Schueler>[] get�berschuss() {
        return �berschuss;
    }

}
