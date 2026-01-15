#!/usr/bin/env ruby

# pclka_mhz = 48
# b = 9600
# big_n = 155
# small_n = 0

# pclka_mhz = 48
# b = 19200
# big_n = 77
# small_n = 0

# pclka_mhz = 48
# b = 38400
# big_n = 38
# small_n = 0


def calc_n(b:, pclka_mhz:, semr: 64, max_error: 1.5)
  big_n=0
  small_n=3
  error = 0

  loop do
    256.times do
      new_error = ((pclka_mhz * (10**6)) / (b * semr * (2**(2*small_n-1)) * (big_n+1))) - 1
      if new_error.abs > error.abs and error != 0
        break
      else
        error = new_error
        big_n += 1
      end
    end
    if error.abs == 1.0
      if small_n == 0
        break
      else
        small_n -= 1
        big_n = 0
        error = 0
      end
    else
      break
    end
  end

  error *= 100.0

  if error.abs <= max_error
    puts "Baud=#{b} SmallN=#{small_n} BigN=#{big_n} Error=%.2f%%" % error
  else
    puts "Baud=#{b} N/A"
  end
end

# semr = [12, 16, 32, 64]


[48/1, 48/2, 48/3, 48/4].each do |pclka_mhz|
  puts "#{pclka_mhz} MHz"
  [300, 1200, 4800, 9600, 19200, 31250, 38400, 51200].each do |b|
    calc_n(b:, pclka_mhz:)
  end
end
