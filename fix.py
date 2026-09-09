import pathlib
p = pathlib.Path("index.html").read_text()
# reemplaza el primer href que contenga Verify o Stripe o #
p = p.replace('href="#"', 'href="https://mpago.la/1YmGoxL"', 1)
# por si tu botón tiene otro formato
if 'mpago.la/1YmGoxL' not in p:
    p = p.replace('Verify My Swarm', '<a href="https://mpago.la/1YmGoxL" target="_blank">Verify My Swarm $49 →</a>')
    # fallback simple: reemplaza todos los # del hero
    p = p.replace('href="#"', 'href="https://mpago.la/1YmGoxL"')

pathlib.Path("index.html").write_text(p)
print("Listo, link insertado")
