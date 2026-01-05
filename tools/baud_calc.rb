#!/usr/bin/env ruby

def calc_n(b:, pclka_mhz:, semr: 64, max_error: 1.5)
  big_n = 0
  small_n = 3
  error = 0

  best=[]

  loop do
    last_error = {error: nil, big_n: nil}

    256.times do
      new_error = ((pclka_mhz.to_f * (10**6)) / (b * semr.to_f * (2**(2*small_n-1)) * (big_n+1))) - 1
      if !last_error[:error].nil? and new_error.abs > last_error[:error].abs
        break
      else
        last_error[:error] =new_error
        last_error[:big_n] = big_n
        big_n += 1
      end
    end
    best.push( {small_n:}.merge(last_error))
    if small_n == 0
      break
    else
      small_n -= 1
      big_n = 0
      error = 0
    end
  end


  best.sort!{|a,b|
    a[:error].abs <=> b[:error].abs
  }

  best.each_with_index{|item, idx|
    best[idx][:error] *= 100.0
  }

  best = best[0]

  if best[:error].abs <= max_error
    puts "Baud=#{b} SmallN=#{best[:small_n]} BigN=#{best[:big_n]} Error=%.2f%%" % best[:error]
  else
    puts "Baud=#{b} N/A"
  end
end

[ 48/1, 40, 48/2, 48/3, 48/4, 32/1, 32/2 ].each do |pclka_mhz|
  puts "#{pclka_mhz} MHz"

  [ 300, 1200, 2400, 4800, 9600, 19200, 31250, 38400, 51200, 115200 ].each do |b|
    calc_n(b:, pclka_mhz:)
  end
end
