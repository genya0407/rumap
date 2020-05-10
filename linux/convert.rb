require 'json'

class Converter
  def initialize
    @global = {}
    @in_app = {}
    @current_context = @global
  end

  def execute(command)
    { execute: command }
  end

  def remap(key, **arguments)
    case arguments
    in { to: { execute: execute } }
      @current_context.merge!(_remap_execution(key, execute: execute))
    in { to: to, with_modifier: with }
      @current_context.merge!(_remap_key(key, to: to, with: with))
    in { to: to }
      @current_context.merge!(_remap_key(key, to: to, with: []))
    else
      raise "Unexpected action: #{arguments}"
    end
  end

  def window(class_only:)
    @in_app[class_only] ||= {}
    @current_context = @in_app[class_only]
    yield
    @current_context = @global
  end

  def to_json
    JSON.generate(
      global: @global,
      in_app: @in_app,
    )
  end

  def _remap_key(key, to:, with:)
    {
      key => { to: to, with: with }
    }
  end

  def _remap_execution(key, execute:)
    {
      key => { execute: execute }
    }
  end
end

puts Converter.new.instance_eval(File.read ARGV[0]).to_json