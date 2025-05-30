

Operations should run in a chain of responsbility pattern:

1- Planner -> Handler -> in a seperate thread send the plan to all nodes, -> Excute -> return a response to the handler -> to the original person.


2- If fails, return to the handler.

