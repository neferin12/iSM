package com.JP_Studios.Exceptions;

@Deprecated
public class SchuelerLimitErreichtExeption extends Exception {
    /**
     *
     * @param kursname Name des Fehler-Kurses
     */
    public SchuelerLimitErreichtExeption(String kursname) {
        super(kursname);
    }
}
