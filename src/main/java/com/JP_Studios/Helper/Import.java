package com.JP_Studios.Helper;

import com.JP_Studios.DeclarationClasses.GlobalConstants;
import com.JP_Studios.Kurs;
import com.JP_Studios.Schueler;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Scanner;

public abstract class Import {

    /**
     *
     * @param wahlen Die CSV-Datei, die die Schueler und deren Wahlen enthält. Zum Format: Ein Schueler pro Zeile, erste Spalte der Name, die nächsten drei die W-Seminarwahlen, die nächsten drei die P-Seminarwahlen
     * @return Gibt den Dateiinhalt als eine {@link ArrayList} der Klasse {@link Schueler} zurück.
     * @throws FileNotFoundException Datei nicht gefunden
     */
    public static ArrayList<Schueler> importSchuelerFromCSV(File wahlen) throws FileNotFoundException {
        ArrayList<Schueler> schuelers = new ArrayList<>();
        try (BufferedReader br = new BufferedReader(new FileReader(wahlen))) {
            String line;
            while ((line = br.readLine()) != null) {
                if (!line.isEmpty()) {
                    String[] daten = line.split(";");
                    int[] wSeminare = new int[]{Integer.parseInt(daten[1]), Integer.parseInt(daten[2]), Integer.parseInt(daten[3])};
                    int[] pSeminare = new int[]{Integer.parseInt(daten[4]), Integer.parseInt(daten[5]), Integer.parseInt(daten[6])};
                    schuelers.add(new Schueler(daten[0], pSeminare, wSeminare));
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return schuelers;
    }

    /**
     *
     * @param kurse Die CSV-Datei, die die Seminare enthält. Zum Format: Erste Spalte die Namen der W-Seminare, zweite Spalte jeweils deren maximale Teilnehmerzahl. Dritte Spalte die Namen der P-Seminare, vierte Spalte jeweils deren maximale Teilnehmerzahl.
     * @return Gibt ein Array zurück. Dieses enthält die W-Seminare als {@link ArrayList} am Index {@link GlobalConstants#W_SEMINAR} und die P-Seminare als {@link ArrayList} am Index {@link GlobalConstants#P_SEMINAR}
     * @throws FileNotFoundException Datei nicht gefunden
     */
    public static ArrayList<Kurs>[] importKurseFromCSV(File kurse) throws FileNotFoundException {
        ArrayList<Kurs> w = new ArrayList<>();
        ArrayList<Kurs> p = new ArrayList<>();
        Scanner scanner = new Scanner(kurse);
        int i = 0;
        while (scanner.hasNextLine()) {
            i++;
            String[] daten = scanner.nextLine().split(";");
            w.add(new Kurs(daten[0], GlobalConstants.W_SEMINAR, Integer.parseInt(daten[1]),i));
            p.add(new Kurs(daten[2], GlobalConstants.P_SEMINAR, Integer.parseInt(daten[3]),i));
        }
        return new ArrayList[]{w, p};
    }
}
