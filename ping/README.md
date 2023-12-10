# Ping

Une ré-écriture de la commande `ping` en Rust.

## Fonctionnement de ping

### ICMP

La commande ping se base sur le protocole `ICMP` pour envoyer un message à une machine sur le réseau, 
qui répondra avec une réponse. Le protocole `ICMP` est situé au même niveau que le protocole `IP` (couche réseau), 
il sert de contrôle pour les erreur de transmission.

Un packet `ICMP` se base sur la trame `IP`, à laquelle les informations suivantes sont ajoutées:

|      Bit 0-7     |      Bit 8-15    |     Bit 16-23    |     Bit 24-31    |
|:----------------:|:----------------:|:----------------:|:----------------:|
|     Trame IP     |     Trame IP     |     Trame IP     |     Trame IP     |
| Type de message  |        Code      |     Checksum     |     Checksum     |

Valeurs de la zone type de message pour ping:

 - **0** : réponse d'echo (code 0)
 - **8** : demande d'echo (code 0)

### Requêtes ICMP pour ping

Une requête de demande de ping aura un type de message 8, pour la demade d'echo. La couche réseau de la machine 
interrogée retournera automatiquement une réponse `ICMP` avec le type de message à 0.

## Implémentation en Rust

Le crate packet de Rust permet de fabriquer des paquets ICMP de type echo, et de parser les réponses.
C'est ce qui est utilisé dans cette implémentation de ping.

Un socket est créé pour se connecter à la machine distante, avec le type RAW et le protocole ICMPV4 (pour l'utilisation d'IP V4).

### Execution

L'ouverture de socket en mode RAW est limitée aux administrateur uniquement, le programme n'est pas lancé avec la commande `cargo run`, 
mais il est construit puis lancé avec la commande sudo.

La cible `run` du `Makefile` simplifie le lancement:

```
make run
```

## Améliorations possibles

 - tests
 - refacto sortie en fonction
 - gestion controle C
 - gestion des paramètres
 - IP v6