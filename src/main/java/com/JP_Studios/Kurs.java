package com.JP_Studios;

import com.JP_Studios.DeclarationClasses.Comparator.SchuelerNameComparator;
import com.JP_Studios.DeclarationClasses.GlobalConstants;

import java.io.Serializable;
import java.util.ArrayList;

/**
 * Jedes Objekt dieser Klasse repräsentiert ein Seminar
 */
public class Kurs implements Serializable {
    private String name;
    private int pOw;
    private ArrayList<Schueler> schuelers = new ArrayList<>();
    private int maximaleSchueler;
    private int index;

    /**
     * @param name             Name des Kurses
     * @param pOw              Bezeichner ob das gewählte Seminar p oder w  ist. Einsetzbar sind {@link GlobalConstants#W_SEMINAR} und {@link GlobalConstants#P_SEMINAR}
     * @param maximaleSchueler Maximale Anzahl an {@link Schueler Schülern}, die den Kurs besuchen dürfen
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
     * @return Bezeichner ob das gewählte Seminar p oder w  ist. Möglich sind {@link GlobalConstants#W_SEMINAR} und {@link GlobalConstants#P_SEMINAR}
     */
    public int getpOw() {
        return pOw;
    }

    /**
     * @return Gibt Index des Kurses zurück
     */
    public int getIndex() {
        return index;
    }

    /**
     * @return Gibt die {@link Schueler Schüler} als Array zurück, die an dem Kurs teilnehmen
     */
    public ArrayList<Schueler> getSchueler() {
        return schuelers;
    }

    /**
     * @return Anzahl der {@link Schueler Schüler} in dem Kurs
     */
    public int getSchuelerZahl() {
        return schuelers.size();
    }

    /**
     * @return Maximale Anzahl an Schülern im Kurs
     */
    int getMaximaleSchueler() {
        return maximaleSchueler;
    }

    /**
     * @param schueler Hier kann ein {@link Schueler Schueler} zum Seminar hinzugefügt werden
     * @param forceAdd Wenn {@code true}, wird das hinzufügen des {@link Schueler Schülers} erzwungen
     * @return Gibt zurück, ob das Hinzufügen des Schülers erfolgreich war
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
     * Sortiert die Schüler des Kurses nach Alphabet
     */
    void sort() {
        schuelers.sort(new SchuelerNameComparator());
    }
}
