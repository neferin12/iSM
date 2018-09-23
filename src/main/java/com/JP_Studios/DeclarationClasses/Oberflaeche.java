package com.JP_Studios.DeclarationClasses;

import com.JP_Studios.Verteiler;

import java.net.SocketAddress;

/**
 * Created by Julian Pollinger
 */
public interface Oberflaeche {
    /**
     * Methode, um den Fortschritt des Controllers an die Oberflaeche zu übermitteln
     * @param p Fortschritt in Prozent
     */
    void handleProgress(int p);


    void verteilenFinished(Verteiler bester);


    void client_connected();

    void server_newClientConnected(int port, SocketAddress remoteSocketAddress);

    void client_connecting(int port);
}
