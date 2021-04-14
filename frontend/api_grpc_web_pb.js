/**
 * @fileoverview gRPC-Web generated client stub for spreadsheet
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');

const proto = {};
proto.spreadsheet = require('./api_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.spreadsheet.SpreadsheetAPIClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.spreadsheet.SpreadsheetAPIPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.spreadsheet.InsertCellsRequest,
 *   !proto.spreadsheet.InsertCellsResponse>}
 */
const methodDescriptor_SpreadsheetAPI_InsertCells = new grpc.web.MethodDescriptor(
  '/spreadsheet.SpreadsheetAPI/InsertCells',
  grpc.web.MethodType.UNARY,
  proto.spreadsheet.InsertCellsRequest,
  proto.spreadsheet.InsertCellsResponse,
  /**
   * @param {!proto.spreadsheet.InsertCellsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.spreadsheet.InsertCellsResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.spreadsheet.InsertCellsRequest,
 *   !proto.spreadsheet.InsertCellsResponse>}
 */
const methodInfo_SpreadsheetAPI_InsertCells = new grpc.web.AbstractClientBase.MethodInfo(
  proto.spreadsheet.InsertCellsResponse,
  /**
   * @param {!proto.spreadsheet.InsertCellsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.spreadsheet.InsertCellsResponse.deserializeBinary
);


/**
 * @param {!proto.spreadsheet.InsertCellsRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.spreadsheet.InsertCellsResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.spreadsheet.InsertCellsResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.spreadsheet.SpreadsheetAPIClient.prototype.insertCells =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/spreadsheet.SpreadsheetAPI/InsertCells',
      request,
      metadata || {},
      methodDescriptor_SpreadsheetAPI_InsertCells,
      callback);
};


/**
 * @param {!proto.spreadsheet.InsertCellsRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.spreadsheet.InsertCellsResponse>}
 *     Promise that resolves to the response
 */
proto.spreadsheet.SpreadsheetAPIPromiseClient.prototype.insertCells =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/spreadsheet.SpreadsheetAPI/InsertCells',
      request,
      metadata || {},
      methodDescriptor_SpreadsheetAPI_InsertCells);
};


module.exports = proto.spreadsheet;

