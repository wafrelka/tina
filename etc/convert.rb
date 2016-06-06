require 'csv'


def convert(raw_file_path, dest_file_path)

	raw = File.read(raw_file_path)

	lines = raw.split("\n")
	items = lines.map{ |l| l.split(' ') }.flatten

	data = items.each_slice(2)
		.map { |raw_code, raw_name|
			if raw_code.empty? || raw_name.empty?
				nil
			else
				[raw_code, raw_name]
			end
		}
		.compact()
		.sort()

	CSV.open(dest_file_path, "w") do |csv|

		data.each do |code, name|
			csv << [code, name]
		end

	end

end


if __FILE__ == $0

	convert("raw_area_code.txt", "area_code.csv")
	convert("raw_epicenter_code.txt", "epicenter_code.csv")

end
