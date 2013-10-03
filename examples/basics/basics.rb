require 'resque'

class Basic
  @queue = :basic_queue
end

Resque.enqueue(Basic, 'some', 'args')
