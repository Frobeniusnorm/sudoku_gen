// TODO: forcing chains
/*
 * Per empty cell: array of possibilities
 * for each empty cell {
 *  for each possibility {
 *      write possibility;
 *      calculate changes for other cells;
 *      for each cell that is certain {
 *          remember certain value for cell; 
 *          write certain value;
 *          calculate changes for other cells;
 *      }
 *      undo writing;
 *  }
 *  for each cell that was certain {
 *      if all certain values for each possibility the same {
 *          write value to cell;
 *          return recurse;
 *      }
 *  }
 * }
 */
