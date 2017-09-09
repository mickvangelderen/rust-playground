#include <iostream>
#include <fstream>
#include <vector>
#include <cstddef>
#include <ios>
#include <iomanip>

int main(int argc, char * argv[])
{
  if (argc < 2) { return -1; }

  int counts[256] = {};

  {
    std::ifstream ifs(argv[1], std::ifstream::binary);
    std::vector<char> buffer (4096,0);

    while (ifs.good())
    {
      ifs.read(buffer.data(), buffer.size());
      std::streamsize read_count = ifs.gcount();

      for (int i = 0; i < read_count; i++)
      {
        // Interpret the byte as an unsigned integer.
        unsigned char byte = static_cast<unsigned char>(buffer[i]);

        // Use the byte value as an index. The two-step casting is necessary.
        counts[static_cast<size_t>(byte)] += 1;
      }
    }

    ifs.close();
  }

  for (int i = 0; i < 256; i++)
  {
    std::cout << "0x" << std::hex << std::setw(2) << std::setfill('0') << i << ": " << std::dec << counts[i] << std::endl;
  }
}
