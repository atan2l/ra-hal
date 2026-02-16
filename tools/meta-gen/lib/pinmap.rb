require 'yaml'
require 'nokogiri'

require_relative 'meta'

PinConfig = Data.define(:pin_count, :pfunc) do
  def initialize
    super(pin_count: Set.new, pfunc: [])
  end

  def add(pin_count:, pfunc:)
    self.pin_count.add(pin_count)
    self.pfunc.concat([*pfunc]) unless pfunc.nil?
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

      pinmap.xpath('/pinMappings/device/components[@id="peripherals"]/components/component').each do |peripheral|
        peripheral.xpath('./pins/pin').each do |pin|
          pin.xpath('./configurations/configuration[@type="connection"][1]').each do |config|
            config.xpath('./alt[@type != "none" or not(@type)]').each do |alt_config|
              if alt_config[:name] =~ /P\d{3}/
                query_args = {pin: alt_config[:name].downcase, pfunc_id: alt_config[:id]}

                alt_query = '/pinMappings/device/connections/connection/altRef[@refId="%<pfunc_id>s"]/parent::connection/altRef'
                alt_names = pinmap.xpath(alt_query % query_args).map do |x|
                  x[:refId]
                end

                query_args[:alt_args] = alt_names.map do |alt|
                  "@id='%s'" % alt
                end.join(' or ')

                pfunc = []

                pfunc_query = '/pinMappings/device/components[@id="ports"]/components/component[@id="%<pin>s"]/pins/pin[@id="%<pin>s"]/configurations/configuration/alt[%<alt_args>s]/registerSetting[@mask="PIN_CFG_MODE_MASK"][1]'
                pfunc.push pinmap.xpath(pfunc_query % query_args)[0]&.attr(:value)

                debug_query = '/pinMappings/device/components[@id="ports"]/components/component[@type="port" and @id="%<pin>s"]/pins/pin/configurations/configuration[@id="%<pin>s"]/alt/registerSetting[@value="IOPORT_PERIPHERAL_DEBUG"]'
                pfunc.push pinmap.xpath(debug_query % query_args)[0]&.attr(:value)

                @peripherals[peripheral[:id]][config[:name]][alt_config[:name]].add(pin_count:, pfunc: pfunc.compact)
                @pin_counts.add(pin_count)
              end
            end
          end
        end
      end

      pinmap.xpath('/pinMappings/device/components[@id="ports"]/components').each do |port|
        port.xpath('./component[@type="port"]').each do |pin|
          pfuncs = []
          pfuncs.push pin.xpath('./configurations/configuration[@name="IRQ"]/alt/registerSetting[@value="IOPORT_CFG_IRQ_ENABLE"]/parent::alt')[0]&.attr(:name)
          pfuncs.push pin.xpath('./configurations/configuration[@name="Pull up"]/alt/registerSetting[@value="IOPORT_CFG_PULLUP_ENABLE"]')[0]&.attr(:value)
          pfuncs.push pin.xpath('./configurations/configuration[@name="Output type"]/alt/registerSetting[@value="IOPORT_CFG_NMOS_ENABLE"]')[0]&.attr(:value)
          pfuncs.push pin.xpath("./pins/pin/configurations/configuration[@name='#{pin[:name]}']/alt/registerSetting[@value='IOPORT_PERIPHERAL_DEBUG']")[0]&.attr(:value)
      
          @peripherals['port'][port[:id]][pin[:id]].add(pin_count:, pfunc: pfuncs.compact)
          @pin_counts.add(pin_count)
        end
      end

    end

    def finish
      @peripherals.each do |_, peri|
        peri.each do |_, pins|
          pins.each do |pin, configs|
            pfunc = configs.pfunc.uniq
            # This is an error because in theory each peripheral
            # should only have one function for a pin.  Multiple
            # peripheral functions per pin is allowed.
            # if pfunc.length > 1
            #   STDERR.puts configs.inspect
            #   STDERR.puts pfunc.inspect
            #   throw "More than one peripheral function, unsure what to do."
            # end
            # pfunc = pfunc[0]
            pins[pin] = {
              "pin_count" => configs.pin_count.to_a.sort
            }
            unless pfunc.empty?
              pins[pin]["pfunc"] = pfunc
            end
          end
        end
      end

      @peripherals.to_yaml
    end
  end
end
