require 'yaml'
require 'nokogiri'

require_relative 'meta'

module Meta
  class Peripherals < Meta::MetaItem
    Match = /.+\.svd/
    OutFile = 'peripherals.yaml'

    def initialize
      @peripherals = Set.new()
    end

    def is_match?(path)
      path =~ Match
    end

    def process(name:, data:)
      puts "PERIPHERALS: Reading #{name}"

      svd = Nokogiri::XML::Document.parse(data)
      svd.remove_namespaces!
  
      svd.xpath('/device/peripherals/peripheral').each do |peripheral|
        name = peripheral.xpath('./name[1]')
        @peripherals.add(name.text)
      end
    end

    def finish
      p = { "peripherals" => @peripherals.to_a.sort }
      p.to_yaml
    end
  end
end
