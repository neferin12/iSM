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

public abstract class Export {
    /**
     * @return Ausgabe der Ergebnisse als Text
     */
    public static String getResultsAsText(Verteiler verteiler) {
        ArrayList<Schueler>[] überschuss = verteiler.getÜberschuss();
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

        results += System.lineSeparator() + System.lineSeparator() + "______________Schüler_ohne_Seminar______________ "+ System.lineSeparator()+"W-Seminare:"+ System.lineSeparator();
        for (Schueler schueler : überschuss[W_SEMINAR]) {
            results += "        "+ schueler.name+ System.lineSeparator();
        }
        results += "P-Seminare:" + System.lineSeparator();
        for (Schueler schueler : überschuss[GlobalConstants.P_SEMINAR]) {
            results += "        "+ schueler.name+ System.lineSeparator();
        }

        results += System.lineSeparator() + System.lineSeparator() + System.lineSeparator() + "Mimimiquote: " + verteiler.punktzahl;

        return results;
    }

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
            }

            if (schueler.kurse()[P_SEMINAR] != -1) {
                pSeminar = verteiler.getKurse(P_SEMINAR).get(schueler.kurse()[P_SEMINAR]).getName();
            }
            outputSchueler += schueler.name + ";" + wSeminar + ";" + pSeminar + System.lineSeparator();
        }

        File output = new File(path + "/iSM Export/Schüler.csv");
        output.createNewFile();
        PrintWriter out = new PrintWriter(new PrintStream(output, StandardCharsets.ISO_8859_1));
        out.print(outputSchueler);
        out.flush();
        out.close();

    }
}
