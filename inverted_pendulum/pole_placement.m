clear

%% System Parameters

m = 5; % Pendulum mass
L = 0.5; % Pendulum length
d = 0.5; % Cart damping
M = 20; % Cart mass

g = 9.81; %% Gravity

%% System Matrices

s = 1; % 1 for pendulum up, -1 for pendulum down

A = [0 1 0 0
    0 -d/M -m*g/M 0
    0 0 0 1
    0 -s*d/M*L -s*(m+M)*g/M*L 0]

B = [0
    1/M
    0
    s/M*L]

fprintf("Uncontrolled eigenvalues of A:")
eigs(A)

fprintf("Controllability Matrix:")
ctrb(A,B)

fprintf("Rank of ctrb")
rank(ctrb(A, B))

fprintf("Desired eigenvalues:")
poles = [-1,-1.5,-1.6,-1.7]

fprintf("Gain Matrix:")
K = place(A, B, poles)
    