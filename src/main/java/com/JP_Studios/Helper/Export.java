package com.JP_Studios.Helper;

import com.JP_Studios.DeclarationClasses.GlobalConstants;
import com.JP_Studios.Kurs;
import com.JP_Studios.Schueler;
import com.JP_Studios.Verteiler;

import java.io.File;
import java.io.IOException;
import java.io.PrintStream;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;

import static com.JP_Studios.DeclarationClasses.GlobalConstants.P_SEMINAR;
import static com.JP_Studios.DeclarationClasses.GlobalConstants.W_SEMINAR;

/**
 * Klasse zum exportieren der Ergebnisse
 */
public abstract class Export {
    /**
     * @param verteiler Der {@link Verteiler}, dessen Ergebnisse ausgegeben werden sollen
     * @return Ausgabe der Ergebnisse als Text
     */
    @Deprecated
    public static String getResultsAsText(Verteiler verteiler) {
        ArrayList<Schueler>[] �berschuss = verteiler.get�berschuss();
        String results = "";
        results += "______________Seminare______________ " + System.lineSeparator()+"W-Seminare:"+ System.lineSeparator();
        ArrayList<Kurs> WSeminar = verteiler.getKurse(W_SEMINAR);
        for (Kurs kurs : WSeminar) {
            results+="  "+ kurs.getName()+":"+ System.lineSeparator();
            ArrayList<Schueler> schuelers = kurs.getSchueler();
            for (Schueler schueler : schuelers) {
                results += "        "+ schueler.name+ System.lineSeparator();
            }
        }
        results += System.lineSeparator()+"P-Seminare:"+ System.lineSeparator();

        ArrayList<Kurs> PSeminar = verteiler.getKurse(GlobalConstants.P_SEMINAR);
        for (Kurs kurs : PSeminar) {
            results+="  "+ kurs.getName()+":"+ System.lineSeparator();
            ArrayList<Schueler> schuelers = kurs.getSchueler();
            for (Schueler schueler : schuelers) {
                results += "        "+ schueler.name+ System.lineSeparator();
            }
        }

        results += System.lineSeparator() + System.lineSeparator() + "______________Sch�ler_ohne_Seminar______________ "+ System.lineSeparator()+"W-Seminare:"+ System.lineSeparator();
        for (Schueler schueler : �berschuss[W_SEMINAR]) {
            results += "        "+ schueler.name+ System.lineSeparator();
        }
        results += "P-Seminare:" + System.lineSeparator();
        for (Schueler schueler : �berschuss[GlobalConstants.P_SEMINAR]) {
            results += "        "+ schueler.name+ System.lineSeparator();
        }

        results += System.lineSeparator() + System.lineSeparator() + System.lineSeparator() + "Mimimiquote: " + verteiler.punktzahl;

        return results;
    }

    /**
     * @param verteiler Der {@link Verteiler}, dessen Ergebnisse ausgegeben werden sollen
     * @param path      Der Pfad, zu dem die Dateien ausgegeben werden sollen
     * @throws IOException Falls das Exportieren nicht m�glich war
     */
    public static void saveCSVFiles(Verteiler verteiler, String path) throws IOException {
        ArrayList<Schueler> schuelers = verteiler.getSchueler();
        File folder = new File(path + "/iSM Export");
        if (!folder.exists()) {
            final boolean success = folder.mkdirs();
            if (!success) {
                throw new IOException("Erstellen des Ordners fehlgeschlagen");
            }
        }
        String outputSchueler = "Name;W-Seminar;P-Seminar" + System.lineSeparator();
        for (Schueler schueler : schuelers) {
            String wSeminar = "KEINES";
            String pSeminar = "KEINES";

            if (schueler.kurse()[W_SEMINAR] != -1) {
                wSeminar = verteiler.getKurse(W_SEMINAR).get(schueler.kurse()[W_SEMINAR]).getName();
            } else {
                int w�nsche[] = schueler.getWseminarwahl();
                wSeminar += " (W�nsche: ";
                for (int i : w�nsche) {
                    wSeminar += " " + i + " ";
                }
                wSeminar += ")";
            }

            if (schueler.kurse()[P_SEMINAR] != -1) {
                pSeminar = verteiler.getKurse(P_SEMINAR).get(schueler.kurse()[P_SEMINAR]).getName();
            } else {
                int w�nsche[] = schueler.getPseminarwahl();
                pSeminar += " (W�nsche: ";
                for (int i : w�nsche) {
                    pSeminar += " " + i + " ";
                }
                pSeminar += ")";
            }
            outputSchueler += schueler.name + ";" + wSeminar + ";" + pSeminar + System.lineSeparator();
        }

        File output = new File(path + "/iSM Export/Sch�ler.csv");
        output.createNewFile();
        try (PrintWriter out = new PrintWriter(new PrintStream(output, StandardCharsets.ISO_8859_1))) {
            out.print(outputSchueler);
            out.flush();
        }


        folder = new File(path + "/iSM Export/Seminare/P-Seminare");
        if (!folder.exists()) {
            final boolean success = folder.mkdirs();
            if (!success) {
                throw new IOException("Erstellen des Ordners fehlgeschlagen");
            }
        }
        folder = new File(path + "/iSM Export/Seminare/W-Seminare");
        if (!folder.exists()) {
            final boolean success = folder.mkdirs();
            if (!success) {
                throw new IOException("Erstellen des Ordners fehlgeschlagen");
            }
        }

        ArrayList<Kurs>[] kurse = verteiler.getKurse();
        ArrayList<Kurs> pSeminare = kurse[GlobalConstants.P_SEMINAR];
        ArrayList<Kurs> wSeminare = kurse[GlobalConstants.W_SEMINAR];

        for (Kurs kurs : wSeminare) {
            String textOutput = "";
            ArrayList<Schueler> sch�ler = kurs.getSchueler();
            for (Schueler schueler : sch�ler) {
                textOutput += schueler.name + System.lineSeparator();
            }
            output = new File(path + "/iSM Export/Seminare/W-Seminare/" + kurs.getName() + ".csv");
            output.createNewFile();
            try (PrintWriter out = new PrintWriter(new PrintStream(output, StandardCharsets.ISO_8859_1))) {
                out.print(textOutput);
                out.flush();
            }
        }

        for (Kurs kurs : pSeminare) {
            String textOutput = "";
            ArrayList<Schueler> sch�ler = kurs.getSchueler();
            for (Schueler schueler : sch�ler) {
                textOutput += schueler.name + System.lineSeparator();
            }
            output = new File(path + "/iSM Export/Seminare/P-Seminare/" + kurs.getName() + ".csv");
            output.createNewFile();
            try (PrintWriter out = new PrintWriter(new PrintStream(output, StandardCharsets.ISO_8859_1))) {
                out.print(textOutput);
                out.flush();
            }
        }

    }
}
