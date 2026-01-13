#!/usr/bin/env ruby

ofs0=  0xffffffff
ofs1=  0xffffcedf
# 00fffffc 00ffffff 00fffffc 00ffffff 00fffffc 00ffffff 200ffffc 200fffff 407ffffc 407fffff 400dfffc 400dffff ffffffff
secmpu=[
  0x00fffffc,
  0x00ffffff,
  0x00fffffc,
  0x00ffffff,
  0x00fffffc,
  0x00ffffff,
  0x200ffffc,
  0x200fffff,
  0x407ffffc,
  0x407fffff,
  0x400dfffc,
  0x400dffff,
  0xffffffff
]

opts = []

lvdas = ((ofs1 & (0b001 << 2)) >> 2)
lvdas = case lvdas
when 0
  :lvdas_enable
when 1
  :lvdas_disable
else
  panic
end
opts.push(lvdas)

vdsel1 = ((ofs1 & (0b111 << 3)) >> 3)
vdsel1 = case vdsel1
when 0b000
  :vdsel1_384
when 0b001
  :vdsel1_282
when 0b010
  :vdsel_251
when 0b011
  :vdsel_190
when 0b100
  :vdsel_170
else
  panic
end
opts.push(vdsel1)


hocoen= ((ofs1 & (0b001 << 8)) >> 8)
hocoen= case hocoen
when 0b000
  :hoco_enable
when 0b001
  :hoco_disable
else
  panic
end
opts.push(hocoen)



hocofrq1= ((ofs1 & (0b111 << 12)) >> 12)
hocofrq1= case hocofrq1
when 0b000
  :hoco_24mhz
when 0b010
  :hoco_32mhz
when 0b100
  :hoco_48mhz
when 0b101
  :hoco_64mhz
else
  error = "Invalid HOCOFRQ1: 0b%03b" % hocofrq1
  throw Exception.new(error)
end
opts.push(hocofrq1)

puts opts.inspect
