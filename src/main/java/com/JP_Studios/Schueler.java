package com.JP_Studios;


import com.JP_Studios.DeclarationClasses.GlobalConstants;

import java.io.Serializable;

/**
 * Jedes Objekt dieser Klasse repräsentiert einen Schüler
 */
public class Schueler implements Serializable {
    public String name;
    public int punktzahl;
    int[] pseminarwahl;
    int[] wseminarwahl;
    private int[] ergebnis;

    /**
     * @param name         Name des Schuelers
     * @param pseminarwahl 1. und 2. Wahl des Schülers bezüglich des P-Seminars
     * @param wseminarwahl 1. und 2. Wahl des Schülers bezüglich des W-Seminars
     */
    public Schueler(String name, int[] pseminarwahl, int[] wseminarwahl) {
        this.name = name;
        this.pseminarwahl = pseminarwahl;
        this.wseminarwahl = wseminarwahl;
        ergebnis = new int[]{-1, -1};
    }

    /**
     * @param seminar       Seminar, das der Schueler bekommt
     * @param pOw           Bezeichner ob das gewählte Seminar p oder w  ist. Einsetzbar sind {@link GlobalConstants#W_SEMINAR} und {@link GlobalConstants#P_SEMINAR}
     * @param wahlPunktzahl Die Punkte die zur "Glückwertung" des Schülers hinzuzufügen sind. Möglich  sind {@link GlobalConstants#ERSTE_WAHL}, {@link GlobalConstants#ZWEITE_WAHL}, {@link GlobalConstants#DRITTE_WAHL} oder, falls keiner der Wünsche berücksichtigt werden konnte, {@link GlobalConstants#KEINE_WAHL}
     * @param forceAdd      Gibt an, ob das Hinzufügen des {@link Schueler Schülers} zum {@link Kurs} erzwungen werden soll, falls letzterer voll ist
     * @param verteiler     Der Verteiler zu dessen Kurs der Schüler hinzugefügt werden soll
     * @return Gibt zurück, ob das Zuweisen erfolgreich war
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
     * @return Gibt die Indices der Kurs der zurück, die  der Schueler  bekommen hat. Index für die jeweiligen Seminare sind {@link GlobalConstants#W_SEMINAR} und {@link GlobalConstants#P_SEMINAR}
     */
    public int[] kurse() {
        return ergebnis;
    }


    public int[] getPseminarwahl() {
        return pseminarwahl;
    }

    public int[] getWseminarwahl() {
        return wseminarwahl;
    }
}
