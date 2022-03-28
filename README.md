# TempEdu
An time management app

# EduTemp

## US 

Professeur : 
US1 : je veut pouvoir créer des groupe d'éléve
US2 : je peut pouvoir ajouter des créno a un groupe d'éléve
US3 : Je veut pouvoirs assigner un autre professeur a un créno
US4 : Je veut pouvoirs entrer mes disponibiltié sur une semaine
US5 : Je veut que on me previenne quand l'emplois tu temps est pas compatible avec un des participant
US6 : Je veut pouvoirs rendre visible ou non les cours sur une semaine
US7 : je veut avoirs un appercu sur les horaire effectuer par chaque groupe/professeur

Eleve : 
US7 : je veut pouvoirs consulter mon emplois du temps sur plusieur semaine sur téléphone ou ordinateur
US8 : je veux recevoir des notifications de quand une semaine est visible
US9 : je veux recevoir des notifications de quand une heurs change sur une semaine visible
US10 : je veux recevoir mes notifications de la forme que je le souhaite

## Coter technique

Base de données sql
Api possédent des capaciter d'api key
Prise en charge OAuth

Queue de notification avec pulsar
Service de distribution de notification par websocket a part


BD :

User : 
id
username
first_name
last_name
abrv_name
mail

Groupe : 
id
name

UserToGroupe
id_user
id_groupe

Crénaux :
id
startTime
endTime

CrénauxToUser :
id_user
id_crenaux

CrénauxToGroupe : 
id_groupe
id_crenaux

//auto generated with an env option
Week :
id
start_date
end_date

CrénauxToWeek
id_week
id_créneaux

[![](https://mermaid.ink/img/pako:eNqNU0FugzAQ_Irlc_IBrm3VU09N1QsS2uJtsAI2Wi9RqyR_r4kNJGCl-LTeGc8OY3ySpVUoM4n0rGFP0ORG-PXhkMT5vN3a07Xe2VeyXYsiE9-guWiBWLvAjUiajY6LWiMF6hOhge5n4Mbtzl7HTWTxmL0QF0vTc-kKnEjZXYp6ZnL8J-LBw611DhUmKW_A3k0vUoIpKjji0ljkjMbumYPCv-RJ-BTKfvmdhlpoddNi0mYvOg8ZaHAB1OAzTCLwRcc00oCuQ_MyGAnprbIySY6n-2gfn2XdoBfwf12hgHEGoFE37VE23s5q5b5MKN-341codCXplrU1s6nxzlZl4SpLvEJbbmSD5HNX_qlelXPJFXpfMvOlAjrkMjc9r2v7JF6UZksyY-pwI6Fj-_5rymEfOPHBh-blD6EmTUE)](https://mermaid.live/edit#pako:eNqNU0FugzAQ_Irlc_IBrm3VU09N1QsS2uJtsAI2Wi9RqyR_r4kNJGCl-LTeGc8OY3ySpVUoM4n0rGFP0ORG-PXhkMT5vN3a07Xe2VeyXYsiE9-guWiBWLvAjUiajY6LWiMF6hOhge5n4Mbtzl7HTWTxmL0QF0vTc-kKnEjZXYp6ZnL8J-LBw611DhUmKW_A3k0vUoIpKjji0ljkjMbumYPCv-RJ-BTKfvmdhlpoddNi0mYvOg8ZaHAB1OAzTCLwRcc00oCuQ_MyGAnprbIySY6n-2gfn2XdoBfwf12hgHEGoFE37VE23s5q5b5MKN-341codCXplrU1s6nxzlZl4SpLvEJbbmSD5HNX_qlelXPJFXpfMvOlAjrkMjc9r2v7JF6UZksyY-pwI6Fj-_5rymEfOPHBh-blD6EmTUE)

```mm
erDiagram
    User ||--o{ UserToGroupe : fait_partis
    Groupe ||--o{ UserToGroupe : est_lier
    Crenaux ||--o{ CrenauxToUser : est_lier 
    Crenaux ||--o{ CrenauxToGroupe : est_lier 
    User ||--o{ CrenauxToUser : has 
    Groupe ||--o{ CrenauxToGroupe : has
    Crenaux ||--o{ Week : possede
    Crenaux ||--o{ Matiere : can_have
    User ||--o{ MatiereToUser : can_have
    Matiere ||--o{ MatiereToUser : can_have

    User{
        serial id
        string username
        string last_name
        string abrv_name
        string mail
    }

    Groupe{
        serial id
        string name
    }

    Week{
        serial id
        time start_date
        time end_date
    }

    Crenaux{
        serial id
        time start_time
        time end_time
        string description
    }

    Matiere{
        serial id
        string short
        string description
    }
    ```

    https://mermaid.live/edit#pako:eNqNU8tugzAQ_BXL5-QHuLZRTz01VS9IaIs3wQrYaL1ErZL8e5dg8sJK8Wm9M54dxvigS29QZxrp1cKWoMmdkvUZkNTxuFz6w7le-zfyXYsqUxuwXLRAbMPAjUiajYGL2iIN1BdCB93PyI3btT-Pu5LVc_ZEXE1NP0pXEFTK7lRUmMnxX4g7gVsfAhpMUt6BxU0vUoIrKtjj1FjkXIzdM0eFf8lX4cNQ9kt2FmplzU2Lybqt6gRy0OAEqEEyTCLwTfs00oCth-ZpNDKkN8vKVfJyuo_2-Vm2DYqA_HWFAcYHAJ25aV9k4-3MVu7LhPJ9O36FwVCSbdl69zA13tmsLELliWdo64VukCR3I0_1rJxrrlB86UxKA7TLde56Xtf2SayMZU86Y-pwoaFj__HrynE_cOKD19kG6oCnP-4RTYw