# このようなライブラリを source 以下に置いてしまうと以下のエラーが出てしまうので注意
# cannot load such file -- opal

class CustomRenderer < Redcarpet::Render::HTML

  def block_code(code, language)
    Middleman::Syntax::Highlighter.highlight(code, language)
  end
  
  # last process
  def postprocess(text)
    
    text.gsub!(/,{3}(.*?),{3}/m) do
      <<-BOX
        </br>
        <strong>==========================</strong></br>
            #{Regexp.last_match(1)}</br>
        <strong>==========================</strong></br>
      BOX
    end

    return text
  end
end
