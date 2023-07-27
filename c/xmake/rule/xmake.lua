rule("module.test")
  on_load(function (target)
    if not has_config("test") then 
      target:set("enable", false )
      return
    end
    -- 运行目录修改为根目录
    -- target:set("rundir")
    -- 添加测试组
    target:set("group", "test")
  end)