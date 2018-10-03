package com.JP_Studios;


import com.JP_Studios.DeclarationClasses.GlobalConstants;

import java.io.Serializable;

public class Schueler implements Serializable {
    public String name;
    int [] pseminarwahl;
    int [] wseminarwahl;
    public int punktzahl;
    private int[] ergebnis;

    /**
     * @param name Name des Schuelers
     * @param pseminarwahl 1. und 2. Wahl des Sch�lers bez�glich des P-Seminars
     * @param wseminarwahl 1. und 2. Wahl des Sch�lers bez�glich des W-Seminars
     */
    public Schueler(String name, int [] pseminarwahl, int [] wseminarwahl) {
        this.name = name;
        this.pseminarwahl = pseminarwahl;
        this.wseminarwahl = wseminarwahl;
        ergebnis = new int[]{-1, -1};
    }

    /**
     *
     * @param seminar Seminar, das der Schueler bekommt
     * @param pOw Bezeichner ob das gew�hlte Seminar p oder w  ist. Einsetzbar sind {@link GlobalConstants#W_SEMINAR} und {@link GlobalConstants#P_SEMINAR}
     * @param wahlPunktzahl Die Punkte die zur "Gl�ckwertung" des Sch�lers hinzuzuf�gen sind. M�glich  sind {@link GlobalConstants#ERSTE_WAHL}, {@link GlobalConstants#ZWEITE_WAHL}, {@link GlobalConstants#DRITTE_WAHL} oder, falls keiner der W�nsche ber�cksichtigt werden konnte, {@link GlobalConstants#KEINE_WAHL}
     * @return Gibt zur�ck, ob das Zuweisen erfolgreich war
     */
    public boolean kursSetzen(int seminar, int pOw, int wahlPunktzahl, boolean forceAdd, Verteiler verteiler) {
        boolean result = verteiler.getKurse(pOw).get(seminar).addSchueler(this, forceAdd);
        if (result) {
            ergebnis[pOw] = seminar;
            punktzahl += wahlPunktzahl;
        }

        return result;
    }

    void keineWahlBekommen() {
        punktzahl += GlobalConstants.KEINE_WAHL;
    }

    /**
     *
     * @return Gibt die Indices der Kurs der zur�ck, die  der Schueler  bekommen hat. Index f�r die jeweiligen Seminare sind {@link GlobalConstants#W_SEMINAR} und {@link GlobalConstants#P_SEMINAR}
     */
    public int[] kurse() {
        return ergebnis;
    }
}
