class Point
	attr_accessor :x,:y

	def initialize(x,y)
		@x = x
		@y = y
	end
end

class Rover
	attr_accessor :loc, :heading

	def initialize(location, heading=:N)
		@loc = location
		@heading = heading
	end

	def turn_left
		case @heading
		when :N
			@heading = :E
		when :E
			@heading = :S
		when :S
			@heading = :W
		when :W
			@heading = :N
		end
	end

	def turn_right
		case @heading
		when :N
			@heading = :W
		when :W
			@heading = :S
		when :S
			@heading = :E
		when :E
			@heading = :N
		end
	end

	def forward
		case @heading
		when :N
			@loc.y += 1
		when :W
			@loc.x += 1
		when :S
			@loc.y -= 1
		when :E
			@loc.x -= 1
		end
	end

	def to_s
		super.to_s
		puts "Rover Position = (#{@loc.x}, #{@loc.y}) heading #{@heading}"
	end
end

class Mars
	def initialize(width, height)
		@width = width
		@height = height
		@rovers = []
	end

	def add_rover(initial_pos, heading)
		@rovers << Rover.new(initial_pos, heading)
		return @rovers[@rovers.length-1]
	end

	def move_rover(rover, cmds)
		# keep move code in Mars class as it can detect collisions with other rovers
		cmds.split("").each do |c|
			case c
			when 'L'
				rover.turn_left
			when 'R'
				rover.turn_right
			when 'M'
				rover.forward
				if rover.loc.x < 0 or rover.loc.x > @width or rover.loc.y < 0 or rover.loc.y > @height
					raise "Rover Crash!!!"
				end
			else
				raise "Unknown Command to Rover: #{c}"
			end
		end
	end

	def print_rovers
		for rover in @rovers
			puts rover
		end
	end
end

mars = Mars.new(5, 5)
rover = mars.add_rover(Point.new(1,2), :N)
mars.move_rover(rover, "LMLMLMLMM")
rover = mars.add_rover(Point.new(3,3), :E)
mars.move_rover(rover, "MMRMMRMRRM")
rover = mars.add_rover(Point.new(0,0), :S)
mars.move_rover(rover, "MMMMM")
mars.print_rovers
