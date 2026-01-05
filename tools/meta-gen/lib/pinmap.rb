require 'yaml'
require 'nokogiri'

require_relative 'meta'

PinConfig = Data.define(:pin_count, :pfunc) do
  def initialize
    super(pin_count: Set.new, pfunc: [])
  end

  def add(pin_count:, pfunc:)
    self.pin_count.add(pin_count)
    self.pfunc.push(pfunc) unless pfunc.nil?
  end
end

module Meta
  class PinMap < Meta::MetaItem
    Match = /\.mcu\/\.pinmapping\/PinCfgR7FA[A-Z0-9]\w+\.xml/
    OutFile = 'pinmap.yaml'

    def initialize
      # peri[signal[port[pincount]]]
      @peripherals = Hash.new do |h,k|
        h[k] = Hash.new do |h,k|
          h[k]=Hash.new do |h,k|
            h[k] = PinConfig.new
          end
        end
      end

      @pin_counts = Set.new()
    end

    def is_match?(path)
      path =~ Match
    end

    def process(name:, data:)
      puts "PINMAP: Reading #{name}"

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

                @peripherals[component[:id]][config[:name]][port[:name]].add(pin_count:, pfunc: pfunc[0]&.attr(:value))
                @pin_counts.add(pin_count)
              end
            end
          end
        end
      end

      pinmap.xpath('/pinMappings/device/components[@id="ports"]/components').each do |port|
        port.xpath('./component[@type="port"]').each do |pin|
          irq = pin.xpath('./configurations/configuration[@name="IRQ"]/alt/registerSetting[@value="IOPORT_CFG_IRQ_ENABLE"]/parent::alt')[0]&.attr(:name)
      
          @peripherals['port'][port[:id]][pin[:id]].add(pin_count:, pfunc: irq)
          @pin_counts.add(pin_count)
        end
      end

    end

    def finish
      @peripherals.each do |_, peri|
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

      @peripherals.to_yaml
    end
  end
end



# if File.stat(pack).directory?
#   Dir.foreach(pack) do |dirent|
#     entry = File.join(pack,dirent)
#     next unless File.stat(entry).file?
#     if dirent =~ /PinCfgR7FA[A-Z0-9]\w+\.xml/
#       process_pin_map(entry, File::read(entry))
#     end
#   end
# else
#   Zip::File.open(pack) do |pack|
#     pack.each do |entry|
#       # puts entry.name
#       if entry.name =~ /\.mcu\/\.pinmapping\/PinCfgR7FA[A-Z0-9]\w+\.xml/
#         process_pin_map(entry.name, entry.get_input_stream.read)
#       end
#     end
#   end
# end

# $peripherals.each do |_, peri|
#   peri.each do |_, pins|
#     pins.each do |pin, configs|
#       pfunc = configs.pfunc.uniq
#       if pfunc.length > 1
#         throw "NOO"
#       end
#       pfunc = pfunc[0]
#       pins[pin] = {
#         "pin_count" => configs.pin_count.to_a.sort,
#         "pfunc" => pfunc
#       }
#     end
#   end
# end

# # $peripherals['_pin_counts'] = $pin_counts.to_a.sort
# puts $peripherals.to_yaml
