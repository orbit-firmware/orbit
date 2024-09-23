const Modifier = {
  LeftControl: 0x0100,
  RightControl: 0x1100,
  LeftShift: 0x0200,
  RightShift: 0x1200,
  LeftAlt: 0x0400,
  RightAlt: 0x1400,
  LeftGui: 0x0800,
  RightGui: 0x1800,
};

function lc(code) {
  return code | Modifier.LeftControl;
}

function rc(code) {
  return code | Modifier.RightControl;
}

function r(code) {
  return lc(rc(code));
}

function ls(code) {
  return code | Modifier.LeftShift;
}

function rs(code) {
  return code | Modifier.RightShift;
}

function s(code) {
  return ls(rs(code));
}

function la(code) {
  return code | Modifier.LeftAlt;
}

function ra(code) {
  return code | Modifier.RightAlt;
}

function a(code) {
  return la(ra(code));
}

function lg(code) {
  return code | Modifier.LeftGui;
}

function rg(code) {
  return code | Modifier.RightGui;
}

function g(code) {
  return lg(rg(code));
}

export default {
  "lc": lc,
  "rc": rc,
  "r": r,
  "ls": ls,
  "rs": rs,
  "s": s,
  "la": la,
  "ra": ra,
  "a": a,
  "lg": lg,
  "rg": rg,
  "g": g,
};