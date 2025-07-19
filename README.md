# Corrector

Corrector de català. Passa un text i el torna corretgit. Res mes que un wraper a la api de SofCatalà.
Només funcionen les opcions de retornar el json `--json` i la de corretgir amb lo més probable `--fuck-it`

# Ús
```bash
Corrector ortogràfic de català per a la terminal. (Powered by SofCatalà)

Usage: corrector [OPTIONS] <TEXT>

Arguments:
  <TEXT>  Text a corretgir

Options:
  -f, --fuck-it  No preguntis, corretgeix amb el més probable
  -j, --json     No facis res, només imprimeix el resultat en json
  -h, --help     Print help  
```

## TODO

- [ ] Implementar la correcció interactiva
