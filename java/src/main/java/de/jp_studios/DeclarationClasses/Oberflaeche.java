package de.jp_studios.DeclarationClasses;

import de.jp_studios.Verteiler;

/**
 * Interface zum Implementieren einer Oberfläche
 */
public interface Oberflaeche {
    /**
     * Methode, um den Fortschritt des Controllers an die Oberflaeche zu übermitteln
     *
     * @param p Fortschritt in Prozent
     */
    void handleProgress(int p);

    /**
     * Methode, die ausgeführt wird, wenn der Verteilvorgang abgschlossen ist
     *
     * @param bester Der beste {@link Verteiler}
     */
    void verteilenFinished(Verteiler bester);
}
