#include <gb/drawing.h>
#include <gb/gb.h>
#include <gb/cgb.h>
#include <stdio.h>
#include <stdlib.h>

#ifdef GAMEBOYCOLOR
  #define SYSTEM_ID 1
#else
  #define SYSTEM_ID 0
#endif

#define COMM_IO_OFFSET 0x70

void send(uint8_t value)
{
    //printf("sending %d\n", value);

    _IO[COMM_IO_OFFSET] = value;
}

uint8_t receive()
{
    uint8_t value = _IO[COMM_IO_OFFSET];
    return value;
}

uint16_t receive_word()
{
  uint16_t value = receive();
  value <<= 8;
  value |= receive();
  return value;
}

enum Command
{
  DrawText = 0,
  LoadTiles,
  SetBackgroundTiles,
  SetSpriteTile,
  MoveSprite
};

void command_draw_text()
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

void command_load_tiles()
{
  uint8_t is_background = receive();
  uint16_t tile_start_index = receive_word();
  uint16_t tile_count = receive_word();

  // NOTE: using a large 32*32*16 buffer crashes the game, so we do it tile-after-tile instead
  uint8_t tiles_data[16];

  for (int tile_index = 0; tile_index < tile_count; ++tile_index)
  {
    // TEMP
    LCDC_REG = LCDC_REG | LCDCF_BG8000;
    /*if (tile_index > 30) {
      LCDC_REG = LCDC_REG & ~LCDCF_BG8000;
    }*/

    for (int i = 0; i < 16; ++i)
    {
      tiles_data[i] = receive();
    }

    if (is_background == 1)
    {
      set_bkg_data(tile_start_index + tile_index, 1, tiles_data);
    }
    else
    {
      set_sprite_data(tile_start_index + tile_index, 1, tiles_data);
    }
  }
}

void command_set_background_tiles()
{
  uint8_t tile_x = receive();
  uint8_t tile_y = receive();
  uint8_t tile_w = receive();
  uint8_t tile_h = receive();

  //uint8_t is_attribute = receive();
  uint16_t tile_count = tile_w * tile_h;

  uint8_t tiles_indices[20 * 18]; // TODO Indices or attributes, depending on the selected mode // TODO use malloc?

  for (int i = 0; i < tile_count; ++i)
  {
    tiles_indices[i] = receive();
  }

  //VBK_REG = is_attribute;
  set_bkg_tiles(tile_x, tile_y, tile_w, tile_h, tiles_indices);
}

void command_set_sprite_tile() // TODO multi tiles
{
  uint8_t sprite_index = receive();
  uint8_t tile_index = receive();

  set_sprite_tile(sprite_index, tile_index);
}

void command_move_sprite()
{
  uint8_t sprite_index = receive();
  uint8_t x = receive();
  uint8_t y = receive();

  move_sprite(sprite_index, x, y);
}

void send_inputs()
{
  send(joypad());
}

void receive_commands()
{
  uint16_t command_count = receive_word();

//disable_interrupts();

  for (uint16_t i = 0; i < command_count; ++i)
  {
    uint8_t command_id = receive();

    switch (command_id)
    {
      case DrawText: command_draw_text(); break;
      case LoadTiles: command_load_tiles(); break;
      case SetBackgroundTiles: command_set_background_tiles(); break;
      case SetSpriteTile: command_set_sprite_tile(); break;
      case MoveSprite: command_move_sprite(); break;

      default:
        printf("unknown command id: %d\n", command_id);

        wait_vbl_done();
        return;
    }
  }

//enable_interrupts();
}

const uint16_t palette[] =
{
  18, 24, 31, 0
};

void main()
{
  DISPLAY_ON;
  SHOW_BKG;
  SHOW_SPRITES;

  send(SYSTEM_ID);

  while (1)
  {
    send_inputs();
    receive_commands();

  SHOW_SPRITES;
    wait_vbl_done();
  SHOW_SPRITES;
  }
}
