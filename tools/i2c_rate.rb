#!/usr/bin/env ruby

def calc(brl:, brh:, th: 300, tr: 300, div: 1, pclkb: 48, nf: 1)
  div = div.to_f
  pclkb = pclkb * 1000000.0
  brl = brl.to_f
  brh = brh.to_f
  th = th.to_f
  tr = tr.to_f
  iic_clk = pclkb / div

  if div == 1
    n = 3
  else
    n = 2
  end

  puts 1.0 / (
    ( \
      (brh+n+nf) + (brl+n+nf) \
    ) \
    / \
    (iic_clk+tr+th)
  )
end

# Am I mathing this wrong because these values seem way off from what the RM claims.

puts "100 kHz, pclk=32"
calc(brl: 15, brh: 12, div: 8, th: 1000, pclkb: 32)
puts

puts "400 kHz, pclk=32"
calc(brl: 17, brh: 6, div: 2, pclkb: 32)
puts

puts "100 kHz, pclk=48"
calc(brl: 18, brh: 15, div: 16, th: 1000, pclkb: 48)
puts

puts "100 kHz, pclk=48"
calc(brl: 26, brh: 25, div: 4, th: 1000)
puts

puts "400 kHz, pclk=48"
calc(brl: 24, brh: 15, div: 1)
