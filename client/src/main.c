#include <gb/gb.h>
#include <gb/drawing.h>
#include <stdio.h>
#include <stdlib.h>

#define COMM_IO_OFFSET 0x70

void send(uint8_t value)
{
    //printf("sending %d\n", value);

    _IO[COMM_IO_OFFSET] = value;
}

uint8_t receive()
{
    uint8_t value = _IO[COMM_IO_OFFSET];

    //printf("received %d\n", value);

    return value;
}

enum Command
{
  ClearScreen = 0,
  ClearRect,
  DrawPoint,
  DrawLine,
  DrawCircle,
  PrintText
};

void command_clear_screen()
{
  box(0, 0, 159, 143, M_FILL);
}

void command_clear_rect()
{
  uint8_t x = receive();
  uint8_t y = receive();
  uint8_t w = receive();
  uint8_t h = receive();

  color(WHITE, WHITE, SOLID);
  box(x, y, x + w, x + h, M_FILL);
}

void command_draw_point()
{
  uint8_t x = receive();
  uint8_t y = receive();

  color(BLACK, WHITE, SOLID);
  plot_point(x, y);
}

void command_draw_line()
{
  uint8_t x1 = receive();
  uint8_t y1 = receive();
  uint8_t x2 = receive();
  uint8_t y2 = receive();

  line(x1, y1, x2, y2);
}

void command_draw_circle()
{
  uint8_t x = receive();
  uint8_t y = receive();
  uint8_t r = receive();

  color(BLACK, WHITE, SOLID);
  //circle(x, y, r, M_FILL);
  plot_point(x, y);
}

void command_print_text()
{
  uint8_t x = receive() / 8; // units = pixel to tile
  uint8_t y = receive() / 8;

  uint8_t text[100]; // TODO max size?

  uint8_t size = receive();
  for (uint8_t i = 0; i < size; ++i)
  {
    text[i] = receive();
  }

  text[size] = '\0';

  gotogxy(x, y);
  gprint(text);
}

void send_inputs()
{
  send(joypad());
}

void receive_commands()
{
  uint8_t command_count = receive();

  for (uint8_t i = 0; i < command_count; ++i)
  {
    uint8_t command_id = receive();

    switch (command_id)
    {
      case ClearScreen: command_clear_screen(); break;
      case ClearRect: command_clear_rect(); break;
      case DrawPoint: command_draw_point(); break;
      case DrawLine: command_draw_line(); break;
      case DrawCircle: command_draw_circle(); break;
      case PrintText: command_print_text(); break;

      default:
        printf("unknown command id: %d\n", command_id);
    }
  }
}

void main()
{
  while (1)
  {
    send_inputs();
    receive_commands();
    wait_vbl_done();
  }
}
