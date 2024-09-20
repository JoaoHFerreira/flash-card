# Todo in the dev order


8. flash_methods
    8.3 Discover how to perform joins and print then


11. Add while for menu:
    11.1. Menu must have:
        11.1.1. Insert Part;
        11.1.2. Tests Part;
        11.1.3. Statistcs Part;
12. Export CSV for statistics
13. Connect though API
14. Flash Card Mechanic
        make the user be able to choose how many questions he want to answer for..
        RECREATE TABLES.. - use the opportunity to
        create 
        created_at in all
        add in historical_acceptances
            answer_given
            switch is right ofr answer_rate
        14.1.3. In the future the user wiill be able to train based in a given subject:
    14.2. Pontuation. Since is about personal devlopment the user will define their ponctuation with the flow below:
        14.2.5. Based on the user schore the next practicce_schedule will be scheduled;
            14.2.5.1 Algorithm for that will be defined;
        14.2.6. The result wiill be stored in the historical_acceptances table to see further consults;

After run todos
1. All tables are using serial for ids, i need to understand how to properly use UUID
2. Add created_at and updated_at to the tables;
3. Beautify the code;
4. Add error handlers;
5. Convert it into API
6. AI features
7. Since is concepts the historical part should be a range and not a bool true and falase.