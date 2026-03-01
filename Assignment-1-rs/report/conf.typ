#let conf(doc) = {
  set text(
    font: "New Computer Modern",
    size: 12pt,
  )
  set page(
    paper: "a4",
    margin: (x: 1in, y: 1in),
    numbering: "1"
  )
  set par(
    justify: true,
    leading: 0.52em,
  )

  show title: set text(size: 17pt)
  show title: set align(center)
  show title: set block(below: 2em)

  set document(title: [CS208 Report - Complexity in Action])
  set document(author: "Finlay Robb")

  set heading(numbering: "1.")
  show heading.where(level: 1): set block(above: 2em, below: 1em)

  show figure: set block(above: 1.2em, below: 2em)
  show figure.caption: set text(size: 11pt)
  show figure.caption: set block(width: 80%)

  doc
}
