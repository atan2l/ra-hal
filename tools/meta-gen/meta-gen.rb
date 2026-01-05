#!/usr/bin/env ruby

require 'rubygems'
require 'yaml'
require 'zip'
require 'nokogiri'

if ARGV.length != 2
  STDERR.puts "Need two arugments: IN_PATH OUT_DIR"
  exit 1
end

out_dir = ARGV.pop
pack = ARGV.pop

if out_dir.nil?
  STDERR.puts "Must specify output directory"
  exit 1
end

if pack.nil?
  STDERR.puts "Must specify FSP-PACK"
  exit 1
end

our_file = File.expand_path(__FILE__)
old_pwd = Dir.pwd
Dir.chdir(File.dirname(our_file))
Dir["./**/*.rb"].each do |file|
  next if File.expand_path(file) == our_file
  require_relative(file)
end
Dir.chdir(old_pwd)

meta_items = Meta::MetaItem.descendants.map do |meta_item|
  meta_item.new
end

if File.stat(pack).directory?
  Dir.foreach(pack) do |dirent|
    entry = File.join(pack, dirent)
    next unless File.stat(entry).file?

    meta_items.each do |item|
      if item.is_match?(dirent)
        item.process(name: entry, data: File::read(entry))
      end
    end
  end
else
  Zip::File.open(pack) do |pack|
    pack.each do |entry|
      meta_items.each do |item|
        if item.is_match?(entry.name)
          item.process(name: entry.name, data: entry.get_input_stream.read)
        end
      end
    end
  end
end

meta_items.each do |item|
  out_filename = item.class.const_get('OutFile')
  puts "Writing #{out_filename}"
  out_path = File.join(out_dir, out_filename)
  File.write(out_path, item.finish)
end
