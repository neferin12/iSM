package com.JP_Studios;

import com.JP_Studios.DeclarationClasses.Comparator.SchuelerNameComparator;
import com.JP_Studios.DeclarationClasses.GlobalConstants;

import java.io.Serializable;
import java.util.ArrayList;

/**
 * Jedes Objekt dieser Klasse repr�sentiert ein Seminar
 */
public class Kurs implements Serializable {
    private String name;
    private int pOw;
    private ArrayList<Schueler> schuelers = new ArrayList<>();
    private int maximaleSchueler;
    private int index;

    /**
     * @param name             Name des Kurses
     * @param pOw              Bezeichner ob das gew�hlte Seminar p oder w  ist. Einsetzbar sind {@link GlobalConstants#W_SEMINAR} und {@link GlobalConstants#P_SEMINAR}
     * @param maximaleSchueler Maximale Anzahl an {@link Schueler Sch�lern}, die den Kurs besuchen d�rfen
     * @param i                Index des Kurses in der {@link Verteiler#kurses Kursliste}
     */
    public Kurs(String name, int pOw, int maximaleSchueler, int i) {
        this.name = name;
        this.pOw = pOw;
        this.maximaleSchueler = maximaleSchueler;
        index = i;
    }

    /**
     * @return Name des Kurses
     */
    public String getName() {
        return name;
    }

    /**
     * @return Bezeichner ob das gew�hlte Seminar p oder w  ist. M�glich sind {@link GlobalConstants#W_SEMINAR} und {@link GlobalConstants#P_SEMINAR}
     */
    public int getpOw() {
        return pOw;
    }

    /**
     * @return Gibt Index des Kurses zur�ck
     */
    public int getIndex() {
        return index;
    }

    /**
     * @return Gibt die {@link Schueler Sch�ler} als Array zur�ck, die an dem Kurs teilnehmen
     */
    public ArrayList<Schueler> getSchueler() {
        return schuelers;
    }

    /**
     * @return Anzahl der {@link Schueler Sch�ler} in dem Kurs
     */
    public int getSchuelerZahl() {
        return schuelers.size();
    }

    /**
     * @return Maximale Anzahl an Sch�lern im Kurs
     */
    int getMaximaleSchueler() {
        return maximaleSchueler;
    }

    /**
     * @param schueler Hier kann ein {@link Schueler Schueler} zum Seminar hinzugef�gt werden
     * @param forceAdd Wenn {@code true}, wird das hinzuf�gen des {@link Schueler Sch�lers} erzwungen
     * @return Gibt zur�ck, ob das Hinzuf�gen des Sch�lers erfolgreich war
     */
    boolean addSchueler(Schueler schueler, boolean forceAdd) {
        if (forceAdd) {
            schuelers.add(schueler);
        } else {
            if (schuelers.size() < maximaleSchueler) {
                schuelers.add(schueler);
            } else {
                return false;
            }
        }
        return true;
    }


    /**
     * Sortiert die Sch�ler des Kurses nach Alphabet
     */
    void sort() {
        schuelers.sort(new SchuelerNameComparator());
    }
}
