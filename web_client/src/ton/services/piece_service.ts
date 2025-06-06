import { BaseService } from './base';
import { apiService } from './api_service';
import { tonStateStore } from './state_store';
import { storageService } from './storage_service';

/**
 * Service for interacting with EchoPiece contracts
 */
export class PieceService extends BaseService {
  /**
   * Gets data from a piece contract as raw base64 encoded data
   * @param pieceAddress Piece address
   * @returns Piece data
   */
  async getPieceData(pieceAddress: string): Promise<string | null> {
    if (!pieceAddress) {
      return null;
    }

    try {
      const result = (await apiService.callContractGetter(pieceAddress, 'getData')) as any;

      if (!(result && result.success && result.stack && result.stack.length > 0)) {
        return null;
      }

      const cellData = result.stack[0].cell;
      if (!cellData) {
        return null;
      }

      try {
        const cell = apiService.parseCell(cellData);
        const slice = cell.beginParse();
        const result = slice.loadStringTail();
        return result;
      } catch (parseError) {
        this.logError(`Processing piece data for ${pieceAddress}`, parseError);
        return null;
      }
    } catch (error) {
      this.logError(`Getting piece data for ${pieceAddress}`, error);
      return null;
    }
  }

  /**
   * Fetches data for all pieces, updates the state store, and caches the result.
   * @param userAddress The address of the current user for caching.
   * @param pieceAddresses Array of piece addresses to fetch.
   */
  fetchAllPieceData(userAddress: string, pieceAddresses: string[] | null): void {
    if (!userAddress || !pieceAddresses || pieceAddresses.length === 0) {
      return;
    }

    pieceAddresses.forEach(async (address) => {
      try {
        const data = await this.getPieceData(address);
        const currentState = tonStateStore.getState();
        const newPieceData = { ...(currentState.pieceData || {}), [address]: data };
        tonStateStore.updateState({ pieceData: newPieceData });
        storageService.savePieces(userAddress, newPieceData); // Cache the updated data
      } catch (error) {
        this.logError(`Processing piece ${address}`, error);
        const currentState = tonStateStore.getState();
        // Store null to indicate a fetch attempt failed for this address
        const newPieceData = { ...(currentState.pieceData || {}), [address]: null };
        tonStateStore.updateState({ pieceData: newPieceData });
        storageService.savePieces(userAddress, newPieceData); // Also cache failures
      }
    });
  }
}

// Export a singleton instance for use across the application
export const pieceService = new PieceService();
