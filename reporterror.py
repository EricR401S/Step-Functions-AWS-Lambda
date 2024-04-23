import json

def lambda_handler(event, context):
    # TODO implement
    
    assert("input_string" not in event)
    
    message = "You did not use the key -input_string- for your json input. It is needed for the letter counter"
        
    
    return {
        'statusCode': 200,
        'body': json.dumps(message)
    }
