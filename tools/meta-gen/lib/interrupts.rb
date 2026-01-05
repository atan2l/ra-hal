require 'yaml'
require 'nokogiri'

require_relative 'meta'

module Meta
  class Interrupts < Meta::MetaItem
    Match = /.+\.svd/
    OutFile = 'interrupts.yaml'

    def initialize
      @interrupts = Set.new()
    end

    def is_match?(path)
      path =~ Match
    end

    def process(name:, data:)
      puts "IRQ: Reading #{name}"
      svd = Nokogiri::XML::Document.parse(data)
      svd.remove_namespaces!
  
      svd.xpath('/device/peripherals/peripheral/interrupt').each do |peripheral|
        name = peripheral.xpath('./name[1]').text
        @interrupts.add(name)
      end
    end

    def finish
      p = { "interrupts" => @interrupts.to_a.sort }
      p.to_yaml
    end
  end
end

