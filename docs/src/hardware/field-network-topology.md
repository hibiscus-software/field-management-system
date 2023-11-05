# Field Network Topology

The network topology of our FMS implementation is pretty much exactly the same as the official FIRST one but with slightly different hardware. I suggest reading the FMS whitepaper if you would like to understand it more in depth. You can find that [here](https://fms-manual.readthedocs.io/en/latest/fms-whitepaper/fms-whitepaper.html).

```mermaid
flowchart TB
    subgraph FMS[Field Management System]
        subgraph FCC[Field Control Cabinet]
        A(Field Switch):::nodes===B(Field Router):::nodes
        A===C(Field PLC):::nodes
        A===D(Field Access Point):::nodes
        A===G(Field Server):::nodes
        C===E(Field Stack Light):::nodes
        C===F(Field E-Stop):::nodes
        end
        subgraph RED_SCC[Red Station Control Cabinet]
        H(Red SCC Switch):::nodes===I(Red Remote I/O):::nodes
        H===P(Red Driver Station 1):::nodes
        H===Q(Red Driver Station 2):::nodes
        H===R(Red Driver Station 3):::nodes
        I===J(Red Stack Light 1):::nodes
        I===K(Red Stack Light 2):::nodes
        I===L(Red Stack Light 3):::nodes
        I===M(Red E-Stop 1):::nodes
        I===N(Red E-Stop 2):::nodes
        I===O(Red E-Stop 3):::nodes
        end
        subgraph BLUE_SCC[Blue Station Control Cabinet]
        S(Blue SCC Switch):::nodes===T(Blue Remote I/O):::nodes
        S===U(Blue Driver Station 1):::nodes
        S===V(Blue Driver Station 2):::nodes
        S===W(Blue Driver Station 3):::nodes
        T===X(Blue Stack Light 1):::nodes
        T===Y(Blue Stack Light 2):::nodes
        T===Z(Blue Stack Light 3):::nodes
        T===AA(Blue E-Stop 1):::nodes
        T===AB(Blue E-Stop 2):::nodes
        T===AC(Blue E-Stop 3):::nodes
        end
       A===H
       A===S
    end

    %% Styles %%
    classDef nodes fill:#d3d3d3,stroke:#333,stroke-width:4px,color:#333

    %% FMS Styles %%
    style FMS fill:#d3d3d3,stroke:#333,stroke-width:4px,color:#333

    %% FCC Styles %%
    style FCC fill:#bcf5bc,stroke:#333,stroke-width:4px,color:#333
    linkStyle 0 stroke:#0aa582
    linkStyle 1 stroke:#0aa582
    linkStyle 2 stroke:#0aa582
    linkStyle 3 stroke:#0aa582
    linkStyle 4 stroke:#f0ce35
    linkStyle 5 stroke:#f0ce35
    linkStyle 26 stroke:#0aa582
    linkStyle 27 stroke:#0aa582
    
    %% Red SCC Styles %%
    style RED_SCC fill:#ff9a9a,stroke:#333,stroke-width:4px,color:#333
    linkStyle 6 stroke:#0aa582
    linkStyle 7 stroke:#0aa582
    linkStyle 8 stroke:#0aa582
    linkStyle 9 stroke:#0aa582
    linkStyle 10 stroke:#f0ce35
    linkStyle 11 stroke:#f0ce35
    linkStyle 12 stroke:#f0ce35
    linkStyle 13 stroke:#f0ce35
    linkStyle 14 stroke:#f0ce35
    linkStyle 15 stroke:#f0ce35

    %% Blue SCC Styles %%
    style BLUE_SCC fill:#add8e6,stroke:#333,stroke-width:4px,color:#333
    linkStyle 16 stroke:#0aa582
    linkStyle 17 stroke:#0aa582
    linkStyle 18 stroke:#0aa582
    linkStyle 19 stroke:#0aa582
    linkStyle 20 stroke:#f0ce35
    linkStyle 21 stroke:#f0ce35
    linkStyle 22 stroke:#f0ce35
    linkStyle 23 stroke:#f0ce35
    linkStyle 24 stroke:#f0ce35
    linkStyle 25 stroke:#f0ce35
```