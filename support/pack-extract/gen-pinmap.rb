#!/usr/bin/env ruby

require 'rubygems'
require 'yaml'
require 'zip'
require 'nokogiri'

pack=ARGV.pop
# pack="/Users/alex/arm/fsp-packs/internal/projectgen/ra/packs/Renesas.RA_mcu_ra4m1.6.3.1.pack"

PinConfig = Data.define(:pin_count, :pfunc) do
  def initialize
    super(pin_count: Set.new, pfunc: [])
  end

  def add(pin_count:, pfunc:)
    self.pin_count.add(pin_count)
    self.pfunc.push(pfunc)
  end
end

# peri[signal[port[pincount]]]
$peripherals = Hash.new do |h,k|
  h[k] = Hash.new do |h,k|
    h[k]=Hash.new do |h,k|
      h[k] = PinConfig.new
    end
  end
end
$pin_counts = Set.new()

def process_pin_map(name,data)
  pinmap = Nokogiri::XML::Document.parse(data)
  pinmap.remove_namespaces!
  
  pin_count = pinmap.xpath('/pinMappings/device/package/pinLayout/pin').length

  pinmap.xpath('/pinMappings/device/components[@id="peripherals"]/components/component').each do |component|
    component.xpath('./pins/pin').each do |pin|
      pin.xpath('./configurations/configuration[1]').each do |config|
        config.xpath('./alt[@type != "none" or not(@type)]').each do |port|
          if port[:name] =~ /P\d{3}/

            query_args = {pin: port[:name].downcase, pfunc_id: port[:id]}

            alt_query = '/pinMappings/device/connections/connection/altRef[@refId="%<pfunc_id>s"]/parent::connection/altRef'
            alt_names = pinmap.xpath(alt_query % query_args).map do |x|
              x[:refId]
            end

            query_args[:alt_args] = alt_names.map do |alt|
              "@id='%s'" % alt
            end.join(' or ')

            pfunc_query = '/pinMappings/device/components[@id="ports"]/components/component[@id="%<pin>s"]/pins/pin[@id="%<pin>s"]/configurations/configuration/alt[%<alt_args>s]/registerSetting[@mask="PIN_CFG_MODE_MASK"][1]'
            pfunc_query = pfunc_query % query_args
            pfunc = pinmap.xpath(pfunc_query)
            if !pfunc.empty?
              # throw pfunc[0][:value].inspect
            end

            $peripherals[component[:id]][config[:name]][port[:name]].add(pin_count:, pfunc: pfunc[0]&.attr(:value))
            $pin_counts.add(pin_count)
          end
        end
      end
    end
  end
end

Zip::File.open(pack) do |pack|
  pack.each do |entry|
    # puts entry.name
    if entry.name =~ /\.mcu\/\.pinmapping\/PinCfgR7FA[A-Z0-9]\w+\.xml/
      process_pin_map(entry.name, entry.get_input_stream.read)
    end
  end
end

$peripherals.each do |_, peri|
  peri.each do |_, pins|
    pins.each do |pin, configs|
      pfunc = configs.pfunc.uniq
      if pfunc.length > 1
        throw "NOO"
      end
      pfunc = pfunc[0]
      pins[pin] = {
        "pin_count" => configs.pin_count.to_a.sort,
        "pfunc" => pfunc
      }
    end
  end
end

# $peripherals['_pin_counts'] = $pin_counts.to_a.sort
puts $peripherals.to_yaml
