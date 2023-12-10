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

Le crate packet de Rust permet de fabriquer des paquets `ICMP` de type echo, et de parser les réponses.
C'est ce qui est utilisé dans cette implémentation de ping.

Un socket est créé pour se connecter à la machine distante, avec le type `RAW` et le protocole `ICMPV4` (pour l'utilisation d'IP V4).

### Execution

L'ouverture de socket en mode RAW est limitée aux administrateur uniquement, le programme n'est pas lancé avec la commande `cargo run`, 
mais il est construit puis lancé avec la commande sudo.

La cible `run` du `Makefile` simplifie le lancement:

```
make run
```

### Analyse du réseau

Il est possible d'analyser les traces du réseau avec la commande `tcpdump`, sur l'interface locale ou sur une interface Wifi par exemple:

```
sudo tcpdump -i lo -nXX
sudo tcpdump -i wlp2s0 -nXX
```

Ces commandes sont très utiles pour visualiser le contenu des paquets qui transitent sur le réseau, et analyser les éventuels problèmes de 
création ou parsing des paquets `ICMP` de ping.

## Améliorations possibles

 - Revoir le code pour au minimum créer des fonctions, ou des modules séparés
 - Ajouter des tests unitaires
 - Ajouter une gestion des paramètres d'entrée, pour donner la possibilité de modifier l'intervale d'envoi de paquets, le nombre d'envois, etc
 - Ajouter une gestion de l'arrêt du programme avec la commande control+C
 - Prise en charge de l'IP V6